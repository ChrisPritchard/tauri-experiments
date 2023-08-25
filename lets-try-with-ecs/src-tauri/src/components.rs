use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct City {
    pub name: String,
    pub coords: (f32, f32)
}