#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(not(debug_assertions), deny(dead_code))]
#![feature(conservative_impl_trait)]

extern crate glium_text;
extern crate itertools;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate glium;
extern crate image;
extern crate time;

use glium::glutin::{Event, ElementState, MouseButton, VirtualKeyCode, WindowBuilder};
use glium::{Display, DisplayBuild, Surface};
use itertools::Itertools;
use std::rc::Rc;

mod render_state;
mod card_wrapper;
mod gamestate;
mod cards;
mod point;

pub const CARD_WIDTH: f32 = 286.0;
pub const CARD_HEIGHT: f32 = 395.0;

fn main() {
    let mut screen_size = point::Point::new(1024f32, 800f32);

    let display: Display = WindowBuilder::new()
        .with_vsync()
        .with_dimensions(screen_size.x as u32, screen_size.y as u32)
        .with_title("Original card game pls no stealerino")
        .build_glium()
        .unwrap();

    let text_system = glium_text::TextSystem::new(&display);
    let font = glium_text::FontTexture::new(&display,
                                            std::fs::File::open("assets/Arial.ttf").unwrap(),
                                            24)
            .unwrap();

    let (vertex_buffer, indices) = render_state::RenderState::generate_buffers(&display);

    let program = glium::Program::from_source(&display,
                                              include_str!("../assets/2d_texture_shader.vert"),
                                              include_str!("../assets/2d_texture_shader.frag"),
                                              None)
            .unwrap();

    let mut last_frame_time = time::precise_time_s();

    let mut game_state = gamestate::GameState {
        player: gamestate::Player::new("Trangar"),
        opponent: gamestate::Player::new("ubsan"),
    };

    let mut mouse_position = point::Point::zero();

    game_state.player.original_deck.push(Rc::new(cards::LightElemental { health: 10 }));
    game_state.player.hand.push(card_wrapper::CardWrapper::new(Rc::downgrade(&game_state.player.original_deck[0])));

    game_state.update_card_origins(&screen_size);

    'mainLoop: loop {
        for event in display.poll_events() {
            match event {
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    break 'mainLoop
                }
                Event::MouseMoved(x, y) => {
                    mouse_position = (x, y).into();
                    for card in game_state.iter_mut() {
                        if card.dragging {
                            card.mouse_moved(mouse_position);
                        }
                    }
                }
                Event::Resized(x, y) => {
                    screen_size = (x, y).into();
                    game_state.update_card_origins(&screen_size);
                }
                Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                    for card in game_state.iter_mut().sorted_by(|c1, c2| {
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
                    for card in game_state.iter_mut() {
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

        for card in game_state.iter_mut() {
            card.update(diff);
        }

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);

        {
            let mut render_state = render_state::RenderState {
                window: &display,
                frame: &mut frame,
                screen_dimensions: &screen_size,
                vertex_buffer: &vertex_buffer,
                program: &program,
                indices: &indices,
                text_system: &text_system,
                font: &font,
            };

            for card in game_state.iter_mut() {
                card.draw(&mut render_state);
            }
        }

        frame.finish().unwrap();
    }
}
