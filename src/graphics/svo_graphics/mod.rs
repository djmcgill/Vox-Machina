use graphics::{Instance, Vertex};
use svo;
use svo::SVO;
use std::slice::IterMut;

#[cfg(test)]
mod test;

impl SVO {
    pub fn fill_instances(&self, instances: &mut [Instance]) -> u32 {
        let instances_len = instances.len();
        let mut instance_iter = instances.iter_mut();
        self.fill_instances_helper(&mut instance_iter, [0.0, 0.0, 0.0], 1.0);
        (instances_len - instance_iter.len()) as u32
    }

    fn fill_instances_helper(&self, instances_iter: &mut IterMut<Instance>, origin: [f32; 3], side_len: f32) {
        match self {
            &SVO::Voxel{ data } if data.voxel_type == 0 => {},
            &SVO::Voxel{..} => {
                *instances_iter.next().unwrap() = Instance { // Deliberately panic when the array is not long enough
                    translate: origin,                       // TODO: dynamically extend the array somehow?
                    scale: side_len,
                }
            },
            &SVO::Octants(ref suboctants) => {
                for i in 0..8 {
                    let new_scale: f32 = side_len/2.0;
                    let vec = svo::offset_float(i as u8, new_scale);
                    let new_origin: [f32; 3] = [ // FIXME: so gross
                        origin[0] + vec.x,
                        origin[1] + vec.y,
                        origin[2] + vec.z
                    ];
                    suboctants[i].fill_instances_helper(instances_iter, new_origin, new_scale);
                }
            }
        }
    }
}

macro_rules! vert (($p:expr, $t:expr) => (
    Vertex {
        pos: [$p[0] as f32, $p[1] as f32, $p[2] as f32, 1.0],
        tex_coord: [$t[0] as f32, $t[1] as f32],
    }
));

pub const CUBE_VERTS: [Vertex; 24] = [
    // top (0, 0, 1)
    vert!([-1, -1,  1], [0, 0]),
    vert!([ 1, -1,  1], [1, 0]),
    vert!([ 1,  1,  1], [1, 1]),
    vert!([-1,  1,  1], [0, 1]),
    // bottom (0, 0, -1)
    vert!([-1,  1, -1], [1, 0]),
    vert!([ 1,  1, -1], [0, 0]),
    vert!([ 1, -1, -1], [0, 1]),
    vert!([-1, -1, -1], [1, 1]),
    // right (1, 0, 0)
    vert!([ 1, -1, -1], [0, 0]),
    vert!([ 1,  1, -1], [1, 0]),
    vert!([ 1,  1,  1], [1, 1]),
    vert!([ 1, -1,  1], [0, 1]),
    // left (-1, 0, 0)
    vert!([-1, -1,  1], [1, 0]),
    vert!([-1,  1,  1], [0, 0]),
    vert!([-1,  1, -1], [0, 1]),
    vert!([-1, -1, -1], [1, 1]),
    // front (0, 1, 0)
    vert!([ 1,  1, -1], [1, 0]),
    vert!([-1,  1, -1], [0, 0]),
    vert!([-1,  1,  1], [0, 1]),
    vert!([ 1,  1,  1], [1, 1]),
    // back (0, -1, 0)
    vert!([ 1, -1,  1], [0, 0]),
    vert!([-1, -1,  1], [1, 0]),
    vert!([-1, -1, -1], [1, 1]),
    vert!([ 1, -1, -1], [0, 1]),
];

pub const CUBE_INDICES: [u16; 36] = [
    0,  1,  2,  2,  3,  0, // top
    4,  5,  6,  6,  7,  4, // bottom
    8,  9, 10, 10, 11,  8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // front
    20, 21, 22, 22, 23, 20, // back
];
