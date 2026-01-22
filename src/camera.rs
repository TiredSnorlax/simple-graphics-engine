use crate::Vector3;

pub struct Camera {
    pub position: Vector3,
    pub direction: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        }
    }
}
