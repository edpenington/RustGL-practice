
trait IsVector {
    fn length(&self) -> f32;
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn vector_length(vec : &Vector3) -> f32
    {
        let sumsqrs = (vec.x * vec.x) + (vec.y * vec.y) + (vec.z * vec.z);
        sumsqrs.sqrt()
    }
}
