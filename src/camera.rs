use nalgebra::{Isometry3, Matrix4, Point3, ToHomogeneous, Vector3};

pub struct OverheadCamera {
    view: Matrix4<f32>,
}

impl OverheadCamera {
    pub fn new() -> OverheadCamera {
        let eye = Point3::<f32>::new(1.5, -5.0, 3.0) * 3.0;
        let target = Point3::<f32>::new(0.0, 0.0, 0.0);
        let up = Vector3::<f32>::z();
        // TODO: build from Rotation3::look_at_rh
        OverheadCamera { view: Isometry3::<f32>::look_at_rh(&eye, &target, &up).to_homogeneous() }
    }

    pub fn view(&self) -> Matrix4<f32> {
        self.view
    }
}
