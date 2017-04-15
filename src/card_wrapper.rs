use card_graphics::CardGraphics;
use point::Point;

const BOUNCE_BACK_FACTOR: f32 = 0.005f32;

pub struct CardWrapper<'a> {
    current_position: Point,
    position: Point,

    pub dragging: bool,
    pub drag_offset: Point,
    pub graphics: &'a CardGraphics,
}

impl<'a> CardWrapper<'a> {
    pub fn new(graphics: &CardGraphics) -> CardWrapper {
        CardWrapper {
            position: Point::zero(),
            current_position: Point::zero(),

            dragging: false,
            drag_offset: Point::zero(),
            graphics: graphics
        }
    }

    pub fn size(&self) -> Point { Point::new(::CARD_WIDTH, ::CARD_HEIGHT) }
    pub fn position(&self) -> &Point { &self.current_position }
    pub fn set_position(&mut self, p: Point) {
        self.position = p;
    }
    pub fn contains(&self, p: &Point) -> bool {
        p.between(&self.position, &(self.current_position + self.size()))
    }

    pub fn drag_start(&mut self, mouse_position: Point) {
        self.dragging = true;
        self.drag_offset = self.current_position - mouse_position;
    }
    pub fn drag_end(&mut self) { self.dragging = false; }

    pub fn mouse_moved(&mut self, mouse_position: Point) {
        self.current_position = self.drag_offset + mouse_position;
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.dragging {
            let diff = (self.position - self.current_position) * delta_time * BOUNCE_BACK_FACTOR;
            self.current_position += diff;
        }
    }
}