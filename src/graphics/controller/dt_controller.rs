use std::time;

pub struct DtController {
    last_instant: time::Instant,
}

impl DtController {
    pub fn new() -> Self {
        DtController { last_instant: time::Instant::now(), }
    }

    pub fn update_mut(&mut self) -> f32 {
        let now = time::Instant::now();
        let duration = now.duration_since(self.last_instant);
        let dt = (duration.as_secs() * 1000) as f32 +
                        (duration.subsec_nanos() / 1000_000) as f32; 
        self.last_instant = now;
        dt
    }
}
