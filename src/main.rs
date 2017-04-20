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

mod render_state;
mod card_wrapper;
mod gamestate;
mod constants;
mod cards;
mod point;
mod utils;

use glium::glutin::{Event, ElementState, MouseButton, VirtualKeyCode, WindowBuilder};
use glium::{Display, DisplayBuild, Surface, Program};
use glium_text::{TextSystem, FontTexture};
use gamestate::{GameState, Player};
use render_state::RenderState;
use cards::ResourceType;
use std::fs::File;
use point::Point;

fn main() {
    // Default starting screen size. Will be updated with a resize event only
    let mut screen_size = Point::new(1280.0, 960.0);

    let display: Display = WindowBuilder::new()
        .with_vsync()
        .with_dimensions(screen_size.x as u32, screen_size.y as u32)
        .with_title("Original card game pls no stealerino")
        .build_glium()
        .unwrap();

    let text_system = TextSystem::new(&display);
    let font = FontTexture::new(&display,
                                            File::open("assets/Arial.ttf").unwrap(),
                                            24)
            .unwrap();

    let (vertex_buffer, indices) = RenderState::generate_buffers(&display);

    let program = Program::from_source(&display,
                                              include_str!("../assets/2d_texture_shader.vert"),
                                              include_str!("../assets/2d_texture_shader.frag"),
                                              None)
            .unwrap();

    let mut last_frame_time = time::precise_time_s();
    let mut game_state = GameState::new(Player::new("Trangar"), Player::new("ubsan"));
    let mut mouse_position = Point::zero();

    // Fill a deck with 60 cards, 15 of each type
    for _ in 0..15 {
        game_state.player.original_deck.push(Box::new(cards::LightElemental { health: 10 }));
        game_state.player.original_deck.push(Box::new(cards::BuffCard { }));
        game_state.player.original_deck.push(Box::new(cards::GenericMinion {
            name: String::from("Generic minion"),
            attack: 5,
            health: 5,
            cost: vec![(ResourceType::Red, 3)]
        }));
        game_state.player.original_deck.push(Box::new(cards::DamageSpellCard { }));
    }

    game_state.player.reset_deck();
    for _ in 0..5 {
        game_state.player.draw_card();
    }

    game_state.update_card_origins(&screen_size);

    'mainLoop: loop {
        for event in display.poll_events() {
            match event {
                #[cfg(debug_assertions)]
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    break 'mainLoop
                }
                Event::MouseMoved(x, y) => {
                    mouse_position = (x, y).into();
                    game_state.mouse_moved_to(&mouse_position);
                }
                Event::Resized(x, y) => {
                    screen_size = (x, y).into();
                    game_state.update_card_origins(&screen_size);
                }
                Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                    game_state.mouse_pressed_at(&mouse_position);
                }
                Event::MouseInput(ElementState::Released, _) => {
                    game_state.mouse_released(&screen_size);
                }
                // Event::KeyboardInput(ElementState::Pressed, _, Some(key)) => {
                //     match key {
                //         _ => {}
                //     }
                // }
                // Event::KeyboardInput(ElementState::Released, _, Some(key)) => {
                //     match key {
                //         _ => {}
                //     }
                // }
                Event::Closed => break 'mainLoop,
                _ => {}
            };
        }

        // Calculate the time between now and the previous frame time
        let current_time = time::precise_time_s();
        let diff = ((current_time - last_frame_time) * 1_000.0) as f32;
        last_frame_time = current_time;

        // TODO: Make this more efficient so we don't have to update all lists one by one
        // Maybe use Vec.chain
        for card in &mut game_state.player.hand {
            card.update(diff);
        }

        for card in &mut game_state.player.field {
            card.update(diff);
        }

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);

        {
            // Create a render state that will be passed to all cards
            // This needs to be in a nested block because it has a reference to frame
            // and we're discarding frame with frame.finish() before this goes out of scope
            let mut render_state = RenderState {
                window: &display,
                frame: &mut frame,
                screen_dimensions: &screen_size,
                vertex_buffer: &vertex_buffer,
                program: &program,
                indices: &indices,
                text_system: &text_system,
                font: &font,
            };

            for card in &mut game_state.player.hand {
                card.draw(&mut render_state);
            }
            for card in &mut game_state.player.field {
                card.draw(&mut render_state);
            }

            // If we're dragging a card, draw it again so it's always on the top
            // This does mean we're drawing it twice
            // TODO: see if the check of a card is being drawn is faster than drawing it twice
            if let Some(reference) = game_state.dragging_card {
                if let Some(ref mut card) = game_state.get_card_mut(&reference) {
                    card.draw(&mut render_state);
                }
            }
        }

        frame.finish().unwrap();
    }
}
