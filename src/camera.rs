use nalgebra::{Isometry3, Matrix4, Point3, ToHomogeneous, Vector3};

pub struct OverheadCamera {
    eye: Point3<f32>,
    target: Point3<f32>,
    view: Matrix4<f32>, // Cached from the other parameters
}

impl OverheadCamera {
    pub fn new() -> OverheadCamera {
        let eye = Point3::new(1.5, -5.0, 3.0) * 3.0;
        let target = Point3::new(0.0, 0.0, 0.0);
        let up = Vector3::z();
        // TODO: build from Rotation3::look_at_rh
        OverheadCamera {
            eye: eye,
            target: target,
            view: Isometry3::<f32>::look_at_rh(&eye, &target, &up).to_homogeneous(),
        }
    }

    pub fn view(&self) -> Matrix4<f32> {
        self.view
    }

    pub fn pan(&self, dv: Vector3<f32>) -> OverheadCamera {
        let up = Vector3::z();
        let new_target = self.target + dv;
        let new_view = Isometry3::<f32>::look_at_rh(&self.eye, &new_target, &up).to_homogeneous();
        OverheadCamera {
            target: new_target,
            view: new_view,
            ..*self
        }
    }
}
