use glium::{Display, Frame, Program, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use glium_text::{TextSystem, FontTexture};
use constants::{CARD_WIDTH, CARD_HEIGHT};
use glium::backend::Facade;
use point::Point;

pub struct RenderState<'a> {
    pub window: &'a Display,
    pub frame: &'a mut Frame,
    pub screen_dimensions: &'a Point,
    pub vertex_buffer: &'a VertexBuffer<Vertex>,
    pub indices: &'a NoIndices,
    pub program: &'a Program,
    pub text_system: &'a TextSystem,
    pub font: &'a FontTexture,
}

impl<'a> RenderState<'a> {
    pub fn generate_buffers(display: &Facade) -> (VertexBuffer<Vertex>, NoIndices) {

        let vertex1 = Vertex {
            // bottom left
            position: [0.0, CARD_HEIGHT],
            tex_coords: [0.0, 0.0],
        };
        let vertex2 = Vertex {
            // top left
            position: [0.0, 0.0],
            tex_coords: [0.0, 1.0],
        };
        let vertex3 = Vertex {
            // bottom right
            position: [CARD_WIDTH, CARD_HEIGHT],
            tex_coords: [1.0, 0.0],
        };
        let vertex4 = Vertex {
            // top right
            position: [CARD_WIDTH, 0.0],
            tex_coords: [1.0, 1.0],
        };
        let shape = vec![vertex1, vertex2, vertex3, vertex2, vertex3, vertex4];

        let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);
        (vertex_buffer, indices)
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);