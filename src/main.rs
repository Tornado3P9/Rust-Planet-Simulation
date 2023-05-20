extern crate sdl2;

mod planets;

use planets::Planet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
// (use sdl2::video::Window;)

// Define window dimensions
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const AU: f64 = 149.6e6 * 1000.0; // Define astronomical unit in meters
const G: f64 = 6.67428e-11; // Define gravitational constant
const SCALE: f64 = 250.0 / AU; // Define scaling factor for visualization
const TIMESTAMP: i64 = 3600 * 24; // 1 day (in seconds)

const BLACK: Color = Color::BLACK;
const WHITE: Color = Color::WHITE;
const YELLOW: Color = Color::YELLOW;
const GREY: Color = Color::GREY;
const WHITISH: Color = Color::RGB(255, 233, 182);
const CYAN: Color = Color::CYAN;
const RED: Color = Color::RGB(188, 39, 50);

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Planet Simulation", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(BLACK);
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Define Planets
    let mut sun = Planet::new("sun".to_string(), 0., 0., 1.98892e30, 30, YELLOW);

    let mut mercury = Planet::new("mercury".to_string(), 0.387 * AU, 35., 3.30e23, 8, GREY);
    mercury.set_velocity(47.4 * 1000.);

    let mut venus = Planet::new("venus".to_string(), 0.723 * AU, 60., 4.8685e24, 14, WHITISH);
    venus.set_velocity(35.02 * 1000.);

    let mut earth = Planet::new("earth".to_string(), 1.0 * AU, 50., 5.9742e24, 16, CYAN);
    earth.set_velocity(29.783 * 1000.); // 29.783 km/sec in m/s

    let mut mars = Planet::new("mars".to_string(), 1.524 * AU, 95., 6.39e23, 12, RED);
    mars.set_velocity(24.077 * 1000.);

    // Add all planets to 'list': ownership of each planet is being moved to the vector!
    let mut planets = vec![sun, mercury, venus, earth, mars];

    'running: loop {
        // Set the canvas color to black
        canvas.set_draw_color(BLACK);
        canvas.clear();

        // Listen for a user event
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Draw Planets
        planets.iter().for_each(|object| object.draw(&mut canvas));

        // Present the canvas
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
