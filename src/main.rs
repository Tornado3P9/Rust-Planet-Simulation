extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

// use sdl2::rect::Point;
// use sdl2::render::Renderer;

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

struct Planet {
    name: String,
    distance_to_sun: f64,
    angle: f64,
    mass: f64,
    position: (f64, f64),
    velocity: (f64, f64),
    radius: i32,
    color: Color,
    orbit: Vec<i32>,
}
impl Planet {
    pub fn new(
        name: String,
        distance_to_sun: f64,
        angle: f64,
        mass: f64,
        radius: i32,
        color: Color,
    ) -> Self {
        Self {
            name,
            distance_to_sun,
            angle,
            mass,
            position: (
                distance_to_sun * f64::cos(angle.to_radians()),
                distance_to_sun * f64::sin(angle.to_radians()),
            ),
            velocity: (0., 0.),

            radius,
            color,

            orbit: Vec::new(),
        }
    }

    pub fn set_velocity(&mut self, vel: f64) {
        let p = self.position; // position of planet
        let s = (0.0, 0.0); // position of center (sun)
        let vector = (-(p.1 - s.1), p.0 - s.0); // orthogonal vector
        let n = self.normalize_vector(vector); // vector length = 1
        self.velocity = (vel * n.0, vel * n.1); // vector length = planet velocity
    }

    fn normalize_vector(&self, vector: (f64, f64)) -> (f64, f64) {
        let magnitude = (vector.0.powi(2) + vector.1.powi(2)).sqrt();
        (vector.0 / magnitude, vector.1 / magnitude)
    }
}

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
    let mut earth = Planet::new("earth".to_string(), 1.0 * AU, 50., 5.9742e24, 16, CYAN);
    earth.set_velocity(29.783 * 1000.); // 29.783 km/sec in m/s

    'running: loop {
        canvas.set_draw_color(BLACK);
        canvas.clear();
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
        // The rest of the game loop goes here...

        draw_circle(&mut canvas);

        // End of personal game loop

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_circle(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let mut x = 0;
    let mut y = 20;
    let mut p = 1 - 20;

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    while x <= y {
        canvas.draw_point((x + 400, y + 300)).unwrap();
        canvas.draw_point((-x + 400, y + 300)).unwrap();
        canvas.draw_point((x + 400, -y + 300)).unwrap();
        canvas.draw_point((-x + 400, -y + 300)).unwrap();
        canvas.draw_point((y + 400, x + 300)).unwrap();
        canvas.draw_point((-y + 400, x + 300)).unwrap();
        canvas.draw_point((y + 400, -x + 300)).unwrap();
        canvas.draw_point((-y + 400, -x + 300)).unwrap();

        x += 1;

        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }
}
