use sdl2::pixels::Color;

// Define window dimensions
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const AU: f64 = 149.6e6 * 1000.0; // Define astronomical unit in meters
const SCALE: f64 = 250.0 / AU; // Define scaling factor for visualization

pub struct Planet {
    pub name: String,
    distance_to_sun: f64,
    pub mass: f64,
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    radius: i32,
    color: Color,
    orbit: Vec<(i32, i32)>,
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
        let vector = (p.1 - s.1, -(p.0 - s.0)); // orthogonal vector
        let n = self.normalize_vector(vector); // vector length = 1
        self.velocity = (vel * n.0, vel * n.1); // vector length = planet velocity
    }

    fn normalize_vector(&self, vector: (f64, f64)) -> (f64, f64) {
        let magnitude = (vector.0.powi(2) + vector.1.powi(2)).sqrt();
        (vector.0 / magnitude, vector.1 / magnitude)
    }

    pub fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        // Draw the filled-circle using the midpoint circle algorithm
        let x0: i32 = (self.position.0 * SCALE + (WIDTH / 2) as f64) as i32;
        let y0: i32 = (self.position.1 * SCALE + (HEIGHT / 2) as f64) as i32;
        let mut x = self.radius;
        let mut y = 0;
        let mut err = 0;

        canvas.set_draw_color(self.color);

        while x >= y {
            canvas
                .draw_line((x0 + x, y0 + y), (x0 - x, y0 + y))
                .unwrap();
            canvas
                .draw_line((x0 + y, y0 + x), (x0 - y, y0 + x))
                .unwrap();
            canvas
                .draw_line((x0 - x, y0 - y), (x0 + x, y0 - y))
                .unwrap();
            canvas
                .draw_line((x0 - y, y0 - x), (x0 + y, y0 - x))
                .unwrap();

            y += 1;
            err += 1 + 2 * y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }

        // Draw the orbit line
        self.orbit.push((x0, y0));
        if self.orbit.len() > 1000 {
            self.orbit.remove(0);
        }
        if self.orbit.len() > 2 {
            for i in 0..self.orbit.len() - 1 {
                let start = self.orbit[i];
                let end = self.orbit[i + 1];
                canvas.draw_line(start, end).unwrap();
            }
        }
    }
}

// They say that creating a texture array and working with that is faster than using the
// sdl draw_point or draw_line methods because those are quite slow, even if draw_line is
// already quite a bit faster than draw_point.
// But when using textures one can really turn down the process time dramatically by
// calculating only a quarter of a circle and using the cycle's symmetry to just
// copy the rest.
// The overhead of the draw_point method is so huge, that trying to make the code
// more efficient does not do any difference.
