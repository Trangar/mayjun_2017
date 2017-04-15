extern crate itertools;
extern crate bitflags;
#[macro_use]
extern crate glium;
extern crate image;
extern crate time;

use glium::glutin::{Event, ElementState, MouseButton, VirtualKeyCode, WindowBuilder};
use glium::{Display, DisplayBuild, Surface};
use itertools::Itertools;

mod card_graphics;
mod render_state;
mod card_wrapper;
mod point;

pub const CARD_WIDTH: f32 = 286.0;
pub const CARD_HEIGHT: f32 = 395.0;
pub const CARD_IN_HAND_SPACING: f32 = 100.0;

fn update_card_origins(cards: &mut Vec<card_wrapper::CardWrapper>, screen_size: &point::Point) {
    let left_card = point::Point::new((screen_size.x / 2f32) -
                                      (cards.len() as f32 * CARD_IN_HAND_SPACING / 2f32) -
                                      CARD_WIDTH / 2f32,
                                      screen_size.y - CARD_HEIGHT / 2f32);
    for i in 0..cards.len() {
        cards[i].set_position(left_card + (CARD_IN_HAND_SPACING * i as f32, 0.0).into());
    }
}

fn main() {
    let mut screen_size = point::Point::new(1024f32, 800f32);

    let display: Display = WindowBuilder::new()
        .with_vsync()
        .with_dimensions(screen_size.x as u32, screen_size.y as u32)
        .with_title("Original card game pls no stealerino")
        .build_glium()
        .unwrap();

    let (vertex_buffer, indices) = render_state::RenderState::generate_buffers(&display);

    let program = glium::Program::from_source(&display,
                                              include_str!("../assets/2d_texture_shader.vert"),
                                              include_str!("../assets/2d_texture_shader.frag"),
                                              None)
        .unwrap();

    let mut last_frame_time = time::precise_time_s();

    let graphics = card_graphics::CardGraphics::new(&display,
                                                    &include_bytes!("../assets/264.png")[..]);

    let mut cards = Vec::new();
    cards.push(card_wrapper::CardWrapper::new(&graphics));
    cards.push(card_wrapper::CardWrapper::new(&graphics));
    cards.push(card_wrapper::CardWrapper::new(&graphics));
    cards.push(card_wrapper::CardWrapper::new(&graphics));

    let mut mouse_position = point::Point::zero();

    update_card_origins(&mut cards, &screen_size);

    'mainLoop: loop {
        for event in display.poll_events() {
            match event {
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    break 'mainLoop
                }
                Event::MouseMoved(x, y) => {
                    mouse_position = (x, y).into();
                    for card in &mut cards {
                        if card.dragging {
                            card.mouse_moved(mouse_position);
                        }
                    }
                }
                Event::Resized(x, y) => {
                    screen_size = (x, y).into();
                    update_card_origins(&mut cards, &screen_size);
                }
                Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                    for card in cards.iter_mut().sorted_by(|c1, c2| {
                        use std::cmp::Ordering;
                        let c1_position = c1.position();
                        let c2_position = c2.position();
                        if c2_position.x < c1_position.x {
                            Ordering::Less
                        } else if c2_position.x > c1_position.x {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    }) {
                        if card.contains(&mouse_position) {
                            card.drag_start(mouse_position);
                            break;
                        }
                    }
                }
                Event::MouseInput(ElementState::Released, _) => {
                    for card in &mut cards {
                        card.drag_end();
                    }
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(key)) => {
                    match key {
                        _ => {}
                    }
                }
                Event::KeyboardInput(ElementState::Released, _, Some(key)) => {
                    match key {
                        _ => {}
                    }
                }
                Event::Closed => break 'mainLoop,
                _ => {}
            };
        }

        let current_time = time::precise_time_s();
        let diff = ((current_time - last_frame_time) * 1_000.0) as f32;
        last_frame_time = current_time;

        for card in &mut cards {
            card.update(diff);
        }

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);

        {
            let mut render_state = render_state::RenderState {
                frame: &mut frame,
                screen_dimensions: &screen_size,
                vertex_buffer: &vertex_buffer,
                program: &program,
                indices: &indices,
            };

            for card in &cards {
                card.graphics.render(&card.position(), &mut render_state);
            }
        }

        frame.finish().unwrap();
    }
}
