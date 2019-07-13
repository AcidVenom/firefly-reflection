#[derive(Copy, Clone)]
pub struct Vertex2D
{
    pub position : [f32; 2]
}

implement_vertex!(Vertex2D, position);

pub struct Mesh
{
    pub vertex_buffer : glium::VertexBuffer<Vertex2D>,
    pub index_buffer : glium::IndexBuffer<u16>
}

impl Mesh
{
    //---------------------------------------------------------------------------------------------------
    pub fn new(display : &glium::Display, vertices : Vec<Vertex2D>, indices : Vec<u16>) -> Mesh
    {
        Mesh
        {
            vertex_buffer: glium::VertexBuffer::new(display, &vertices).unwrap(),
            index_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap()
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn create_quad(display : &glium::Display, half_size : bool) -> Mesh
    {
        let size = if half_size == true { 0.5 } else { 1.0 };
        let vertices = vec![
            Vertex2D { position: [-size, -size] },
            Vertex2D { position: [size, -size] },
            Vertex2D { position: [-size, size] },
            Vertex2D { position: [size, size] }
        ];

        let indices = vec![0, 1, 2, 2, 1, 3];

        Mesh::new(display, vertices, indices)
    }
}

