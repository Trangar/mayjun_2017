use glium::{Blend, DrawParameters, Surface, Rect};
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::Texture2d;
use render_state::RenderState;
use gamestate::CardReference;
use glium_text::TextDisplay;
use point::Point;

const BOUNCE_BACK_FACTOR: f32 = 0.005f32;

pub struct CardWrapper {
    current_position: Point,
    position: Point,

    pub dragging: bool,
    pub drag_offset: Point,
    pub texture: Option<Texture2d>,
    pub card: Box<::cards::Card>,
}

#[derive(Debug)]
pub struct PlayArguments {
    pub position: Option<u8>,
    pub additional_target: Option<CardReference>,
}

#[derive(Debug)]
pub enum DragResponse {
    Nothing,
    Play(PlayArguments),
}

impl CardWrapper {
    pub fn new(card: Box<::cards::Card>) -> CardWrapper {
        CardWrapper {
            position: Point::zero(),
            current_position: Point::zero(),

            dragging: false,
            drag_offset: Point::zero(),
            texture: None,
            card: card,
        }
    }

    pub fn size(&self) -> Point {
        Point::new(::CARD_WIDTH, ::CARD_HEIGHT)
    }
    pub fn position(&self) -> &Point {
        &self.current_position
    }
    pub fn drag_position(&self) -> &Point {
        &self.current_position
    }
    pub fn set_position(&mut self, p: Point) {
        self.position = p;
    }
    pub fn contains(&self, p: &Point) -> bool {
        let half_size = self.size() / 2f32;
        p.between(&(self.position - half_size),
                  &(self.current_position + half_size))
    }

    pub fn drag_start(&mut self, mouse_position: &Point) {
        self.dragging = true;
        self.drag_offset = self.current_position - *mouse_position;
    }
    pub fn drag_end(&mut self) -> DragResponse {
        self.dragging = false;
        DragResponse::Nothing
    }

    pub fn mouse_moved(&mut self, mouse_position: &Point) {
        self.current_position = self.drag_offset + *mouse_position;
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.dragging {
            let diff = (self.position - self.current_position) * delta_time * BOUNCE_BACK_FACTOR;
            self.current_position += diff;
        }
    }

    fn generate_texture(&self, render_state: &mut RenderState) -> Texture2d {
        let texture: Texture2d = Texture2d::empty(render_state.window,
                                                  ::CARD_WIDTH as u32,
                                                  ::CARD_HEIGHT as u32)
                .unwrap();
        {
            let mut frame_buffer: SimpleFrameBuffer =
                SimpleFrameBuffer::new(render_state.window, &texture).unwrap();
            frame_buffer.clear_color(0.0, 0.0, 0.0, 1.0);
            frame_buffer.clear(Some(&Rect {
                                         left: 1,
                                         bottom: 1,
                                         width: ::CARD_WIDTH as u32 - 2,
                                         height: ::CARD_HEIGHT as u32 - 2,
                                     }),
                               Some((1.0, 1.0, 1.0, 1.0)),
                               false,
                               None,
                               None);

            let text = TextDisplay::new(render_state.text_system,
                                        render_state.font,
                                        self.card.name());
            let matrix = [[0.1, 0.0, 0.0, 0.0],
                          [0.0, 0.075, 0.0, 0.0],
                          [0.0, 0.0, 0.1, 0.0],
                          [-0.95, 0.9, 0.0, 1.0]];
            ::glium_text::draw(&text,
                               render_state.text_system,
                               &mut frame_buffer,
                               matrix,
                               (0.0, 0.0, 0.0, 1.0));
            let mut y = 0.7;
            for line in self.card.description().lines() {

                let text = TextDisplay::new(render_state.text_system, render_state.font, line);
                let matrix = [[0.1, 0.0, 0.0, 0.0],
                              [0.0, 0.075, 0.0, 0.0],
                              [0.0, 0.0, 0.1, 0.0],
                              [-0.95, y, 0.0, 1.0]];
                ::glium_text::draw(&text,
                                   render_state.text_system,
                                   &mut frame_buffer,
                                   matrix,
                                   (0.0, 0.0, 0.0, 1.0));
                y -= 0.1;
            }
        }

        texture
    }

    pub fn draw(&mut self, render_state: &mut RenderState) {
        if self.texture.is_none() {
            self.texture = Some(self.generate_texture(render_state));
        }
        if let Some(ref texture) = self.texture {
            let uniforms = uniform! {
                screen_dimensions: render_state.screen_dimensions.to_slice(),
                offset: (self.current_position - Point::from((::CARD_WIDTH, ::CARD_HEIGHT)) / 2f32).to_slice(),
                tex: texture,
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
}