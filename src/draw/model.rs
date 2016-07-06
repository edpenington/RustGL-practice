use draw::vertex::Vertex as Vertex;
use draw::maths::vector::Vector3 as Vector3;

#[derive(Copy, Clone)]
struct Triangle {
    vertices : [Vertex;3]
}

impl Triangle {
    pub fn from_vertices(v1: Vertex, v2: Vertex, v3: Vertex)
    -> Triangle
    {
        let vertices = [v1,v2,v3];
        Triangle {vertices : vertices}
    }
}

struct Model {
    triangles : Vec<Triangle>,
    position : Vector3,
}

impl Model {
    pub fn vertices(&self) -> Vec<Vertex>
    {
        let mut vertices: Vec<Vertex> = Vec::new();
        for tri in &self.triangles {
            vertices.push(tri.vertices[0]);
            vertices.push(tri.vertices[1]);
            vertices.push(tri.vertices[2]);
        }
        vertices
    }

    pub fn indices(&self) -> Vec<u8>
    {
        let mut indices : Vec<u8> = Vec::new();
        for i in 0..self.triangles.len()
        {
            let mut j = 1 as u8;
            j = j-1;
            indices.push(j);
        }
        indices
    }
}
