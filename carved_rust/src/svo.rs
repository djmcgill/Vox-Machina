use nalgebra::*;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq)]
// Each SVO assumes that it's the cube between (0,0,0) and (1,1,1)
pub enum SVO {
    // At the moment a voxel only has a "type".
    Voxel (i32),

    // Each octant is addressed by the bit vector [z, y, x] 
    // where the variables are booleans for if the octant is ABOVE the given axis.
    // For example, the octant at index 6 (b110) is the cube with 0 <= x < 0.5, 
    // 0.5 <= y < 1, and 0.5 <= z < 1.
    // i.e. ((x >= 0.5) << 0) | ((y >= 0.5) << 1) | ((z <= 0.5) << 2)
    Octants ([Box<SVO>; 8]) 
}

impl SVO {
    pub fn new_voxel(block_type: i32) -> SVO {
        SVO::Voxel(block_type)
    }

    pub fn new_octants(octant_types: [i32; 8]) -> SVO {
        let new_boxed_voxel = |ix| Box::new(SVO::new_voxel(octant_types[ix]));
        SVO::Octants([new_boxed_voxel(0), new_boxed_voxel(1), new_boxed_voxel(2), new_boxed_voxel(3),
                      new_boxed_voxel(4), new_boxed_voxel(5), new_boxed_voxel(6), new_boxed_voxel(7)])
    }

    // A SVO where the lower blocks (i.e. where y < 0.5) are filled.
    pub fn floor() -> SVO { 
        SVO::new_octants([1, 1, 0, 0, 1, 1, 0, 0])
    }

    // If the SVO is a Voxel, split it into octants. If it's already octants, panic.
    fn subdivide_voxel(&mut self) {
        match *self {
            SVO::Voxel(block_type) =>
                *self = SVO::new_octants([block_type, block_type, block_type, block_type,
                                          block_type, block_type, block_type, block_type]),
            _ => panic!("subdivide_voxel called on a non-voxel!")
        }
    } 

    // Follow an index, splitting voxels as necessary. The set the block at the target to `Voxel(new_block_type)`.
    // Then go back up the tree, recombining if we've transformed all the octants in a node to the same voxel.
    pub fn set_block_and_recombine(&mut self, index: &[u8], new_block_type: i32) {
        if let Some(block_type) = self.get_voxel_type() {
            if block_type == new_block_type {return;} // nothing to do
        }

        match index.split_first() {
            // Overwrite whatever's here with the new voxel.
            None => *self = SVO::Voxel(new_block_type), 

            // We need to go deeper
            Some((ix, rest)) => { 
                // Voxels get split up
                if self.get_voxel_type().is_some() { self.subdivide_voxel(); } 

                {
                    let ref mut octants = self.get_mut_octants().unwrap();
                    // Insert into the sub_octant
                    octants[*ix as usize].set_block_and_recombine(rest, new_block_type);
                }

                // Then if we have 8 voxels of the same type, combine them.
                if let Some(combined_block_type) = self.get_octants().and_then(SVO::combine_voxels) {
                    *self = SVO::Voxel(combined_block_type);
                }
            }
        }
    }

    // If the SVO is a Voxel, return its contents.
    pub fn get_voxel_type(&self) -> Option<i32> {
        match *self {
            SVO::Voxel(voxel_type) => Some(voxel_type),
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
        let sanitised_dir = ray_dir.normalize().iter().map(|f| sanitise(*f)).collect();
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
            SVO::Voxel(0) => None,
            SVO::Voxel(_) => Some(hit_position),
            SVO::Octants(ref octants) => {
                // work out which voxels are hit in turn, and if they're solid or not
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
                // then it's the same as for the nearer children
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


    // TODO: write tests for this!
    pub fn on_voxels(&self, on_voxel: fn(f32, f32, f32, i32, i32)) {
        let on_voxel_vec = &|offset : Vec3<f32>, depth: i32, voxel_type: i32 |
            { on_voxel(offset.x, offset.y, offset.z, depth, voxel_type) }; 
        self.on_voxels_from(on_voxel_vec, zero(), 1);
    }

    fn on_voxels_from<F>(&self, on_voxel: &F, origin: Vec3<f32>, depth: i32) 
        where F : Fn(Vec3<f32>, i32, i32) {
        match *self {
            SVO::Voxel(voxel_type) => { on_voxel(origin, depth, voxel_type); }
            SVO::Octants(ref octants) => for (ix, octant) in octants.iter().enumerate() {
                let next_depth = depth + 1;
                let offset = above_axis(ix) / next_depth as f32;
                octant.on_voxels_from(on_voxel, origin + offset, next_depth);
            }   
        }
    }
}

fn sorted_ts(ray_origin: f32, inv_ray_dir: f32) -> (f32, f32) {
    let min_cube_extent = 0.;
    let max_cube_extent = 1.;
    let t1 = (min_cube_extent-ray_origin) * inv_ray_dir;
    let t2 = (max_cube_extent-ray_origin) * inv_ray_dir;
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

// returns a vector with either 0. or 1. as its elements
fn above_axis(ix: usize) -> Vec3<f32> {
    Vec3::new((ix & 1) as f32, (ix & 2) as f32, (ix & 4) as f32)
}

fn above_center(v: &Vec3<f32>) -> Vec3<bool> {
    Vec3::new(v.x > 0.5, v.y > 0.5, v.z > 0.5)
}

impl<'a> Index<&'a [u8]> for SVO {
    type Output = SVO;
    fn index(&self, index: &[u8]) -> &SVO {
        match index.split_first() {
            None => self,
            Some((ix, rest)) => match *self {
                SVO::Voxel(_) => panic!("Index {:?} is too long!", index),
                SVO::Octants(ref octants) => octants[*ix as usize].index(rest)
            }                
        }
    }
}

impl<'a> IndexMut<&'a [u8]> for SVO {
    fn index_mut(&mut self, index: &[u8]) -> &mut SVO {
        match index.split_first() {
            None => self,
            Some((ix, rest)) => match *self {
                SVO::Voxel(_) => panic!("Index {:?} is too long!", index),
                SVO::Octants(ref mut octants) => octants[*ix as usize].index_mut(rest)
            }
        }
    }
}
