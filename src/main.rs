#![cfg_attr(not(debug_assertions), deny(dead_code))]

mod card_wrapper;
mod cards;
mod constants;
mod gamestate;
mod point;
mod render_state;
mod utils;

use cards::ResourceType;
use gamestate::{GameState, Player};
use glium::glutin::{
    dpi::LogicalSize, ElementState, Event, EventsLoop, KeyboardInput, MouseButton, VirtualKeyCode,
    WindowBuilder, WindowEvent,
};
use glium::{Display, Program, Surface};
use glium_text::{FontTexture, TextSystem};
use point::Point;
use render_state::RenderState;
use std::{fs::File, time::Instant};

fn main() {
    // Default starting screen size. Will be updated with a resize event only
    let mut screen_size = Point::new(1280.0, 960.0);

    let wb = WindowBuilder::new()
        .with_dimensions(LogicalSize::new(screen_size.x as _, screen_size.y as _))
        .with_title("Original card game pls no stealerino");
    let mut events_loop = EventsLoop::new();

    let cb = glium::glutin::ContextBuilder::new();
    let display: Display = glium::Display::new(wb, cb, &events_loop).unwrap();
    let text_system = TextSystem::new(&display);
    let font = FontTexture::new(&display, File::open("assets/arial.ttf").unwrap(), 24).unwrap();

    let (vertex_buffer, indices) = RenderState::generate_buffers(&display);

    let program = Program::from_source(
        &display,
        include_str!("../assets/2d_texture_shader.vert"),
        include_str!("../assets/2d_texture_shader.frag"),
        None,
    )
    .unwrap();

    let mut last_frame_time = Instant::now();
    let mut game_state = GameState::new(Player::new("Trangar"), Player::new("ubsan"));
    let mut mouse_position = Point::zero();

    // Fill a deck with 60 cards, 15 of each type
    for _ in 0..15 {
        game_state
            .player
            .original_deck
            .push(Box::new(cards::LightElemental { health: 10 }));
        game_state
            .player
            .original_deck
            .push(Box::new(cards::BuffCard {}));
        game_state
            .player
            .original_deck
            .push(Box::new(cards::GenericMinion {
                name: String::from("Generic minion"),
                attack: 5,
                health: 5,
                cost: vec![(ResourceType::Red, 3)],
            }));
        game_state
            .player
            .original_deck
            .push(Box::new(cards::DamageSpellCard {}));
    }

    game_state.player.reset_deck();
    for _ in 0..5 {
        game_state.player.draw_card();
    }

    game_state.update_card_origins(&screen_size);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    #[cfg(debug_assertions)]
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        running = false;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        mouse_position = (position.x, position.y).into();
                        game_state.mouse_moved_to(&mouse_position);
                    }
                    WindowEvent::Resized(new_size) => {
                        screen_size = (new_size.width, new_size.height).into();
                        game_state.update_card_origins(&screen_size);
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    } => {
                        game_state.mouse_pressed_at(&mouse_position);
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Released,
                        ..
                    } => {
                        game_state.mouse_released(&screen_size);
                    }
                    WindowEvent::CloseRequested => running = false,
                    _ => {}
                }
            }
        });

        // Calculate the time between now and the previous frame time
        let elapsed = last_frame_time.elapsed().as_secs_f32();
        last_frame_time = Instant::now();

        // TODO: Make this more efficient so we don't have to update all lists one by one
        // Maybe use Vec.chain
        for card in &mut game_state.player.hand {
            card.update(elapsed);
        }

        for card in &mut game_state.player.field {
            card.update(elapsed);
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
