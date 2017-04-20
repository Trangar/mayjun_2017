use constants::{BOUNCE_BACK_FACTOR, CARD_WIDTH, CARD_HEIGHT};
use glium::{Blend, DrawParameters, Surface, Rect};
use glium::framebuffer::SimpleFrameBuffer;
use glium_text::{self, TextDisplay};
use glium::texture::Texture2d;
use render_state::RenderState;
use point::Point;
use cards::Card;

/// Holds a card at a specific position on the screen
/// Also contains the card's texture
pub struct CardWrapper {
    /// The position that the card is currently at 
    current_position: Point,

    /// The snap-back position that the card is supposed to be at
    position: Point,

    // TODO: Make dragging and drag_offset a tagged enum?
    /// Is this card being dragged?
    pub dragging: bool,

    /// Relative offset between the mouse and the center of the card
    /// Is an undefined value if this card is not being dragged
    pub drag_offset: Point,

    /// The texture of the current card
    /// If this is None, it will be generated on the next frame
    pub texture: Option<Texture2d>,

    /// A reference to the card that this cardwrapper is holding
    pub card: Box<Card>,
}

impl CardWrapper {
    /// Create a new card wrapper at 0/0 for the given card
    pub fn new(card: Box<Card>) -> CardWrapper {
        CardWrapper {
            position: Point::zero(),
            current_position: Point::zero(),

            dragging: false,
            drag_offset: Point::zero(),
            texture: None,
            card: card,
        }
    }

    /// Gets the size on the screen of this card
    pub fn size(&self) -> Point {
        Point::new(CARD_WIDTH, CARD_HEIGHT)
    }
    /// Gets the drag position of the card
    pub fn drag_position(&self) -> &Point {
        &self.current_position
    }
    /// Set the snap-back position of the card
    pub fn set_position(&mut self, p: Point) {
        self.position = p;
    }
    /// Returns true if the given point is within this cards dimensions
    pub fn contains(&self, p: &Point) -> bool {
        let half_size = self.size() / 2f32;
        p.between(&(self.position - half_size),
                  &(self.current_position + half_size))
    }

    /// Make the card draggable based on the given mouse position
    pub fn drag_start(&mut self, mouse_position: &Point) {
        self.dragging = true;
        self.drag_offset = self.current_position - *mouse_position;
    }
    // pub fn drag_end(&mut self) -> DragResponse {
    //     self.dragging = false;
    //     DragResponse::Nothing
    // }

    /// Update the current position based on the mouse position
    pub fn mouse_moved(&mut self, mouse_position: &Point) {
        debug_assert!(self.dragging, "Card is being moved when it's not being dragged");
        self.current_position = self.drag_offset + *mouse_position;
    }

    /// Update a card every tick
    /// `delta_time` is the amount of milliseconds since the last frame
    pub fn update(&mut self, delta_time: f32) {
        if !self.dragging {
            let diff = (self.position - self.current_position) * delta_time * BOUNCE_BACK_FACTOR;
            self.current_position += diff;
        }
    }

    /// Generate the cards texture based on the given card
    fn generate_texture(&self, render_state: &mut RenderState) -> Texture2d {
        let texture: Texture2d =
            Texture2d::empty(render_state.window, CARD_WIDTH as u32, CARD_HEIGHT as u32).unwrap();
        {
            let mut frame_buffer: SimpleFrameBuffer =
                SimpleFrameBuffer::new(render_state.window, &texture).unwrap();
            frame_buffer.clear_color(0.0, 0.0, 0.0, 1.0);
            frame_buffer.clear(Some(&Rect {
                                         left: 1,
                                         bottom: 1,
                                         width: CARD_WIDTH as u32 - 2,
                                         height: CARD_HEIGHT as u32 - 2,
                                     }),
                               Some((1.0, 1.0, 1.0, 1.0)),
                               false,
                               None,
                               None);

            // TODO: Properly calculate the positions and size of the font
            let text = TextDisplay::new(render_state.text_system,
                                        render_state.font,
                                        self.card.name());
            let matrix = [[0.1, 0.0, 0.0, 0.0],
                          [0.0, 0.075, 0.0, 0.0],
                          [0.0, 0.0, 0.1, 0.0],
                          [-0.95, 0.9, 0.0, 1.0]];
            glium_text::draw(&text,
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
                glium_text::draw(&text,
                                 render_state.text_system,
                                 &mut frame_buffer,
                                 matrix,
                                 (0.0, 0.0, 0.0, 1.0));
                y -= 0.1;
            }
        }

        texture
    }

    /// Draw a card to the screen
    pub fn draw(&mut self, render_state: &mut RenderState) {
        // if we have no texture, generate it
        if self.texture.is_none() {
            self.texture = Some(self.generate_texture(render_state));
        }
        if let Some(ref texture) = self.texture {
            let half_size = self.size() / 2f32;
            let uniforms = uniform! {
                screen_dimensions: render_state.screen_dimensions.to_slice(),
                // we draw from the center, so our top-left is a half size away from our center point
                offset: (self.current_position - half_size).to_slice(),
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