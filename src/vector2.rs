use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Vector2((f32, f32));

impl Vector2 {
    pub fn new(values: (f32, f32)) -> Self {
        Self(values)
    }
}

impl Deref for Vector2 {
    type Target = (f32, f32);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
