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
const TIMESTAMP: f64 = 3600. * 24.; // 1 day (in seconds)

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
    let sun = Planet::new("sun".to_string(), 0., 0., 1.98892e30, 30, YELLOW);

    let mut mercury = Planet::new("mercury".to_string(), 0.387 * AU, 35., 3.30e23, 7, GREY);
    mercury.set_velocity(47.4 * 1000.);

    let mut venus = Planet::new("venus".to_string(), 0.723 * AU, 60., 4.8685e24, 12, WHITISH);
    venus.set_velocity(35.02 * 1000.);

    let mut earth = Planet::new("earth".to_string(), 1.0 * AU, 50., 5.9742e24, 13, CYAN);
    earth.set_velocity(29.783 * 1000.); // 29.783 km/sec in m/s

    let mut mars = Planet::new("mars".to_string(), 1.524 * AU, 95., 6.39e23, 9, RED);
    mars.set_velocity(24.077 * 1000.);

    // Add all planets to 'list': ownership of each planet is being moved to the vector!
    let mut planets = vec![sun, mercury, venus, earth, mars];

    'running: loop {
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

        /***************UPDATE POSITION****************/

        for i in 0..planets.iter().len() {
            let mut total_force_x = 0.;
            let mut total_force_y = 0.;

            for j in 0..planets.iter().len() {
                if planets[i].name == planets[j].name {
                    continue;
                }

                let distance_x = planets[j].position.0 - planets[i].position.0;
                let distance_y = planets[j].position.1 - planets[i].position.1;
                let distance = (distance_x.powf(2.0) + distance_y.powf(2.0)).sqrt();

                let force = (G * planets[i].mass) * (planets[j].mass / distance.powf(2.0));
                let theta = distance_y.atan2(distance_x);
                let force_x = theta.cos() * force;
                let force_y = theta.sin() * force;

                total_force_x += force_x;
                total_force_y += force_y;
            }

            // Force = mass * acceleration (F = m * a) => a = F / m
            planets[i].velocity.0 += total_force_x / planets[i].mass * TIMESTAMP;
            planets[i].velocity.1 += total_force_y / planets[i].mass * TIMESTAMP;

            // position += velocity * dt
            planets[i].position.0 += planets[i].velocity.0 * TIMESTAMP;
            planets[i].position.1 += planets[i].velocity.1 * TIMESTAMP;
        }

        /***************POSITION END****************/

        // Set the canvas color to black
        canvas.set_draw_color(BLACK);
        canvas.clear();

        // Draw Planets
        planets
            .iter_mut()
            .for_each(|object| object.draw(&mut canvas));

        // Present the canvas at 60 fps
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
