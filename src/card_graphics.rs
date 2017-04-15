use glium::texture::{RawImage2d, Texture2d};
use glium::{Blend, DrawParameters, Surface};
use render_state::RenderState;
use glium::backend::Facade;
use std::io::Cursor;
use point::Point;
use image;

pub struct CardGraphics {
    pub texture: Texture2d,
}

impl CardGraphics {
    pub fn new(display: &Facade, bytes: &[u8]) -> CardGraphics {
        let image = image::load(Cursor::new(bytes), image::PNG)
            .unwrap()
            .to_rgba();

        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = Texture2d::new(display, image).unwrap();
        CardGraphics { texture: texture }
    }

    pub fn render(&self, position: &Point, render_state: &mut RenderState) {
        let uniforms = uniform! {
            screen_dimensions: render_state.screen_dimensions.to_slice(),
            offset: position.to_slice(),
            tex: &self.texture,
        };
        render_state.frame
            .draw(render_state.vertex_buffer,
                  render_state.indices,
                  render_state.program,
                  &uniforms,
                  &DrawParameters { blend: Blend::alpha_blending(), ..Default::default() })
            .unwrap();
    }
}