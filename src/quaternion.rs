use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Quaternion((f32, f32, f32, f32));

impl Quaternion {
    pub fn new(values: (f32, f32, f32, f32)) -> Self {
        Self(values)
    }
}

impl Deref for Quaternion {
    type Target = (f32, f32, f32, f32);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
