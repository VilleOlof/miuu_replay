use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Vector3((f32, f32, f32));

impl Vector3 {
    pub fn new(values: (f32, f32, f32)) -> Self {
        Self(values)
    }
}

impl Deref for Vector3 {
    type Target = (f32, f32, f32);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
