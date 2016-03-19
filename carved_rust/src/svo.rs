use nalgebra::*;

#[derive(Debug, PartialEq)]
// Each SVO assumes that it's the cube between (0,0,0) and (1,1,1)
pub enum SVO {
    Voxel { voxel_type: i32, external_id: u32 },

    // For a given point (x, y, z), the index of its octant is
    // ((x >= 0.5) << 0) | ((y >= 0.5) << 1) | ((z <= 0.5) << 2)
    Octants ([Box<SVO>; 8])
}

impl SVO {
    pub fn new_voxel(voxel_type: i32, external_id: u32) -> SVO {
        SVO::Voxel { voxel_type: voxel_type, external_id: external_id }
    }

    pub fn new_octants<F>(make_octant: &F) -> SVO where F: Fn(u8) -> SVO {
        SVO::Octants([
            Box::new(make_octant(0)), Box::new(make_octant(1)),
            Box::new(make_octant(2)), Box::new(make_octant(3)),
            Box::new(make_octant(4)), Box::new(make_octant(5)),
            Box::new(make_octant(6)), Box::new(make_octant(7))])
    }

    fn subdivide_voxel<D, R>(&mut self, deregister_voxel: &D, register_voxel: &R,
                             origin: Vec3<f32>, depth: i32)
            where D : Fn(u32), R : Fn(Vec3<f32>, i32, i32) -> u32 {
        *self = match *self {
            SVO::Voxel { external_id, voxel_type } => {
                deregister_voxel(external_id);
                SVO::new_octants(&|ix| {
                    let uid = register_voxel(origin + offset(ix, depth), depth+1, voxel_type);
                    SVO::new_voxel(voxel_type, uid)
                })
            },
            _ => panic!("subdivide_voxel called on a non-voxel!")
        };
    }

    fn recombine_octants<D, R>(&mut self, deregister_voxel: &D, register_voxel: &R,
                               origin: Vec3<f32>, depth: i32, voxel_type: i32)
            where D : Fn(u32), R : Fn(Vec3<f32>, i32, i32) -> u32 {
        *self = match *self {
            SVO::Octants(ref mut octants) => {
                for octant in octants { octant.deregister_all(deregister_voxel); }
                let uid = register_voxel(origin, depth, voxel_type);
                SVO::new_voxel(voxel_type, uid)
            },
            _ => panic!("recombine_octants called on non-octants!")
        }
    }

    fn deregister_all<D>(&mut self, deregister_voxel: &D) where D: Fn(u32) {
        match *self {
            SVO::Voxel { external_id, .. } => deregister_voxel(external_id),
            SVO::Octants (ref mut octants) =>
                for octant in octants { octant.deregister_all(deregister_voxel); }
        }
    }

    // Follow an index, splitting voxels as necessary. The set the block at the target to `Voxel(new_block_type)`.
    // Then go back up the tree, recombining if we've transformed all the octants in a node to the same voxel.
    pub fn set_block_and_recombine<D, R>(&mut self, deregister_voxel: &D, register_voxel: &R,
                                         index: &[u8], new_block_type: i32)
        where D : Fn(u32), R : Fn(Vec3<f32>, i32, i32) -> u32 {
            self.set_block_and_recombine_from(deregister_voxel, register_voxel, index, new_block_type, zero(), 0);

    }

    // TODO: could this recursion pattern be generalised?
    fn set_block_and_recombine_from<D, R>(&mut self, deregister_voxel: &D, register_voxel: &R, index: &[u8], new_block_type: i32, origin: Vec3<f32>, depth: i32)
            where D : Fn(u32), R : Fn(Vec3<f32>, i32, i32) -> u32 {
        if let Some(block_type) = self.get_voxel_type() {
            if block_type == new_block_type {return;} // nothing to do
        }

        match index.split_first() {
            // Overwrite whatever's here with the new voxel.
            None => {
                self.deregister_all(deregister_voxel);
                let uid = register_voxel(origin, depth, new_block_type);
                *self = SVO::new_voxel(new_block_type, uid);
            },

            // We need to go deeper
            Some((&ix, rest)) => {
                // Voxels get split up
                if self.get_voxel_type().is_some() { self.subdivide_voxel(deregister_voxel, register_voxel, origin, depth); }

                {
                    let ref mut octants = self.get_mut_octants().unwrap();
                    // Insert into the sub_octant
                    octants[ix as usize].set_block_and_recombine_from(deregister_voxel, register_voxel, rest, new_block_type, origin + offset(ix, depth), depth+1);
                }

                // Then if we have 8 voxels of the same type, combine them.
                if let Some(combined_block_type) = self.get_octants().and_then(SVO::combine_voxels) {
                    self.recombine_octants(deregister_voxel, register_voxel, origin, depth, combined_block_type);
                }
            }
        }
    }

    // If the SVO is a Voxel, return its contents.
    pub fn get_voxel_type(&self) -> Option<i32> {
        match *self {
            SVO::Voxel { voxel_type, .. } => Some(voxel_type),
            _ => None
        }
    }

    // If the SVO is Octants, return its contents.
    fn get_octants(&self) -> Option<&[Box<SVO>; 8]> {
        match *self {
            SVO::Octants(ref octants) => Some(octants),
            _ => None
        }
    }

    fn get_mut_octants(&mut self) -> Option<&mut [Box<SVO>; 8]> {
        match *self {
            SVO::Octants(ref mut octants) => Some(octants),
            _ => None
        }
    }

    // Cast a ray into the octree and return the position of collision with a non-type-zero voxel (if any).
    // x = t*d + o where t = length of ray.
    // t = (x-o)/d
    // BUT we know that the hit will have be on the boundary on the cube.
    // So for each axis independently, work out the length to hit 0. and 1.
    pub fn cast_ray(&self, ray_origin: Vec3<f32>, ray_dir: Vec3<f32>) -> Option<Vec3<f32>> {
        let eps = 0.001;
        let sanitise = |f: f32| if f.abs() < eps {eps * f.signum()} else {f};
        let sanitised_dir = ray_dir.normalize().iter().cloned().map(sanitise).collect();
        let inv_dir = Vec3::new(1., 1., 1.)/sanitised_dir;
        self.cast_ray_sanitised(ray_origin, sanitised_dir, inv_dir)
    }

    pub fn cast_ray_sanitised(&self, ray_origin: Vec3<f32>, ray_dir: Vec3<f32>, inv_ray_dir: Vec3<f32>) -> Option<Vec3<f32>> {
        let min_dist = 0.;
        let max_dist = 100000.;
        // TODO: can simplify this by mirroring the ray direction so they all point the same way
        let (t_min_x, t_max_x) = sorted_ts(ray_origin.x, inv_ray_dir.x);
        let (t_min_y, t_max_y) = sorted_ts(ray_origin.y, inv_ray_dir.y);
        let (t_min_z, t_max_z) = sorted_ts(ray_origin.z, inv_ray_dir.z);
        let t_min = [t_min_x, t_min_y, t_min_z].iter().cloned().fold(min_dist, f32::max);
        let t_max = [t_max_x, t_max_y, t_max_z].iter().cloned().fold(max_dist, f32::min);
        if t_min > t_max {return None};
        let hit_position = ray_dir * t_min + ray_origin;
        match *self {
            SVO::Voxel { voxel_type, .. } if voxel_type == 0 => None,
            SVO::Voxel { .. } => Some(hit_position),
            SVO::Octants(ref octants) => {
                // work out which voxels are hit in turn, and if they're solid or not
                // TODO: rather than trying dumbly, we could instead calculate which child is hit. Compare speeds?
                let children : [(bool, bool, bool); 8] = {
                    let x = hit_position.x > 0.5;
                    let y = hit_position.y > 0.5;
                    let z = hit_position.z > 0.5;
                    [ (x, y, z),
                      (!x, y, z), (x, !y, z), (x, y, !z),
                      (x, !y, !z), (!x, y, !z), (!x, !y, z),
                      (!x, !y, !z)]
                };

                // TODO: stop throwing away the hit position between iterations - if it's on the "near" edge
                //       then it's the same as for the nearer children
                // TODO: also this should probably take &Vec3<bool> instead.
                let test_child = |(above_x, above_y, above_z): (bool, bool, bool)| -> Option<Vec3<f32>> {
                    let (above_x, above_y, above_z) = (above_x as usize, above_y as usize, above_z as usize);
                    let child_ix = above_x | (above_y<<1) | (above_z<<2);
                    let above_center = Vec3::new(above_x as f32, above_y as f32, above_z as f32);
                    let new_origin = to_child_space(ray_origin, above_center);
                    octants[child_ix].cast_ray_sanitised(new_origin, ray_dir, inv_ray_dir).map (|child_hit: Vec3<f32>| {
                        from_child_space(child_hit, above_center)
                    })
                };

                children.iter().cloned().map(test_child).find(|x| x.is_some()).and_then(|x| x)
            }
        }
    }

    fn combine_voxels(octants: &[Box<SVO>; 8]) -> Option<i32> {
        octants[0].get_voxel_type().and_then( |voxel_type| {
            let mut tail = octants.iter().skip(1);
            if tail.all(|octant| octant.get_voxel_type() == Some(voxel_type)) {
                Some(voxel_type)
            } else {
                None
            }
        })
    }
}

fn sorted_ts(ray_origin: f32, inv_ray_dir: f32) -> (f32, f32) {
    let t1 = (0.-ray_origin) * inv_ray_dir;
    let t2 = (1.-ray_origin) * inv_ray_dir;
    if t1 < t2 { (t1, t2) } else { (t2, t1) }
}

// TODO: test these specifically
fn to_child_space(vec: Vec3<f32>, offsets: Vec3<f32>) -> Vec3<f32> {
    (vec - offsets*0.5)*2.
}

// TODO: test these specifically
fn from_child_space(vec: Vec3<f32>, offsets: Vec3<f32>) -> Vec3<f32> {
    vec*0.5 + offsets*0.5
}

// Returns a vector with either 0. or 1. as its elements
fn above_axis(ix: u8) -> Vec3<f32> {
    Vec3::new((ix & 1) as f32,
              ((ix >> 1) & 1) as f32,
              ((ix >> 2) & 1) as f32)
}


// Returns the new origin of the child at the given index in global space.
fn offset(ix: u8, depth: i32) -> Vec3<f32> {
    above_axis(ix) / ((1 << depth+1) as f32)
}
