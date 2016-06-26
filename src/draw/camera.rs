use maths::vector::Vector3 as Vector3;

type Matrix = [[f32;4];4];

pub struct Camera {
    position : Vector3,
    direction : Vector3,
    up : Vector3,
}



impl Camera {
    pub fn view_matrix(&self) -> Matrix
    {
        
    }
}
