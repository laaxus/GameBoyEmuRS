#![allow(non_snake_case,dead_code)]

mod window;
mod cpu;


use log::{debug, error};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use window::{create_window};
use cpu::{GameBoy};


const SPRITE_WIDTH:usize = 16;
const SPRITE_HEIGHT:usize = 16;
const MAP_WIDTH:usize = 10;
const MAP_HEIGHT:usize = 9;
const SCREEN_WIDTH:usize = 10;
const SCREEN_HEIGHT:usize = 9;
const SCREEN_SIZE:usize = SCREEN_HEIGHT * SCREEN_WIDTH * SPRITE_HEIGHT * SPRITE_WIDTH;

pub fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) =
        window::create_window("GameBoyEmuRS", &event_loop);

    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);

    let mut gameboy = cpu::GameBoy::new();
    let mut pixels = Pixels::new((SCREEN_WIDTH * SPRITE_WIDTH) as u32, (SCREEN_HEIGHT * SPRITE_HEIGHT) as u32, surface_texture)?;
    let mut paused = false;


    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            gameboy.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::P) {
                paused = !paused;
            }
            if input.key_pressed(VirtualKeyCode::Space) {
                // Space is frame-step, so ensure we're paused
                paused = true;
            }

            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                _hidpi_factor = factor;
            }

            window.request_redraw();
        }
    });
}


