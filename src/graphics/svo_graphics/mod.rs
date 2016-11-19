use graphics::{Instance, Vertex};
use nalgebra::Vector3;
use svo;
use svo::SVO;
use std::slice::IterMut;

#[cfg(test)]
mod test;

impl SVO {
    pub fn fill_instances(&self, instances: &mut [Instance], max_height: i32) -> u32 {
        let instances_len = instances.len();
        let mut instance_iter = instances.iter_mut();
        self.fill_instances_helper(&mut instance_iter,
                                   Vector3::new(0.0, 0.0, 0.0),
                                   f32::powi(2.0, max_height));
        let instance_count = instances_len - instance_iter.len();
        assert!(instance_count <= u32::max_value() as usize);
        instance_count as u32
    }

    fn fill_instances_helper(&self,
                             instances_iter: &mut IterMut<Instance>,
                             origin: Vector3<f32>,
                             side_width: f32) {
        match self {
            &SVO::Voxel { data } if data.voxel_type == 0 => {}
            &SVO::Voxel { .. } => {
                *instances_iter.next().unwrap() = Instance {
                    // Deliberately panic when the array is not long enough
                    translate: *origin.as_ref(), // TODO: dynamically extend the array somehow?
                    side_width: side_width,
                }
            }
            &SVO::Octants(ref suboctants) => {
                for i in 0..8 {
                    let new_side_width = side_width / 2.0;
                    let offset = svo::offset_float(i as u8, new_side_width);
                    suboctants[i]
                        .fill_instances_helper(instances_iter, origin + offset, new_side_width);
                }
            }
        }
    }
}

macro_rules! vert (($p:expr, $t:expr) => (
    Vertex {
        pos: [$p[0] as f32, $p[1] as f32, $p[2] as f32],
        tex_coord: [$t[0] as f32, $t[1] as f32],
    }
));

pub const CUBE_VERTS: [Vertex; 24] = [// top
                                      vert!([0, 0, 1], [0, 0]),
                                      vert!([1, 0, 1], [1, 0]),
                                      vert!([1, 1, 1], [1, 1]),
                                      vert!([0, 1, 1], [0, 1]),
                                      // bottom
                                      vert!([0, 1, 0], [1, 0]),
                                      vert!([1, 1, 0], [0, 0]),
                                      vert!([1, 0, 0], [0, 1]),
                                      vert!([0, 0, 0], [1, 1]),
                                      // right
                                      vert!([1, 0, 0], [0, 0]),
                                      vert!([1, 1, 0], [1, 0]),
                                      vert!([1, 1, 1], [1, 1]),
                                      vert!([1, 0, 1], [0, 1]),
                                      // left
                                      vert!([0, 0, 1], [1, 0]),
                                      vert!([0, 1, 1], [0, 0]),
                                      vert!([0, 1, 0], [0, 1]),
                                      vert!([0, 0, 0], [1, 1]),
                                      // front
                                      vert!([1, 1, 0], [1, 0]),
                                      vert!([0, 1, 0], [0, 0]),
                                      vert!([0, 1, 1], [0, 1]),
                                      vert!([1, 1, 1], [1, 1]),
                                      // back
                                      vert!([1, 0, 1], [0, 0]),
                                      vert!([0, 0, 1], [1, 0]),
                                      vert!([0, 0, 0], [1, 1]),
                                      vert!([1, 0, 0], [0, 1])];

pub const CUBE_INDICES: [u16; 36] = [0, 1, 2, 2, 3, 0 /* top */, 4, 5, 6, 6, 7,
                                     4 /* bottom */, 8, 9, 10, 10, 11, 8 /* right */,
                                     12, 13, 14, 14, 15, 12 /* left */, 16, 17, 18, 18, 19,
                                     16 /* front */, 20, 21, 22, 22, 23, 20 /* back */];
