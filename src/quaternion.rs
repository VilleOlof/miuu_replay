#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(values: (f32, f32, f32, f32)) -> Self {
        Self {
            x: values.0,
            y: values.1,
            z: values.2,
            w: values.3,
        }
    }
}
