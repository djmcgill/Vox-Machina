use svo::SVO;

pub struct SvoController {
    pub svo: SVO,
    pub max_height: i32,
}

impl SvoController {
    pub fn new() -> Self {
        SvoController {
            svo: SVO::example(),
            max_height: 5,
        }
    }
}
