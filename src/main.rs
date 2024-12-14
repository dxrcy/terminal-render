use termrender::shapes::Circle;
use termrender::shapes::Rect;
use termrender::terminal;
use termrender::Point;
use termrender::{Drawable as _, Program};

fn main() {
    let mut app = App::new();
    loop {
        app.tick();
    }
}

struct App {
    frame_number: usize,
    balls: Vec<Ball>,
}

impl App {
    pub fn new() -> Self {
        let balls = vec![Ball::new(Point::new(40.0, 5.0), 15.0)];

        Self {
            frame_number: 0,
            balls,
        }
    }
}

impl Program for App {
    const FPS: u32 = 20;

    fn update(&mut self) {
        self.frame_number += 1;

        for ball in &mut self.balls {
            let velocity_terminal = 50.0;
            let velocity_increase = 0.3;
            let bound_decrease = 0.95;
            let ground_height = 50.0;

            ball.velocity.y = (ball.velocity.y + velocity_increase).min(velocity_terminal);

            if ball.origin.y + ball.radius + ball.velocity.y > ground_height {
                if ball.velocity.y > bound_decrease {
                    ball.velocity.y = bound_decrease - ball.velocity.y;
                    ball.radius = (ball.radius - 3.0).max(1.0);
                } else {
                    ball.velocity.y = 0.0;
                }
            }

            ball.origin.y += ball.velocity.y;
        }
    }

    fn render(&mut self) {
        terminal::clear();

        let rect = Rect::new(0, 50, 80, 10);
        rect.draw();

        for ball in &self.balls {
            let origin = Point::new(ball.origin.x.round() as i32, ball.origin.y.floor() as i32);
            let circle = Circle::new(origin, ball.radius.round() as u32);
            circle.draw();
        }

        // let circle = Circle::new(Point::new(80, 5), self.frame_number as u32 / 10 % 4);
        // circle.draw();
    }
}

struct Ball {
    origin: Point<f32>,
    radius: f32,
    velocity: Point<f32>, // TODO: Create `Vec2<f32>`
}

impl Ball {
    pub fn new(origin: Point<f32>, radius: f32) -> Self {
        Self {
            origin,
            radius,
            velocity: Point::new(0.0, 0.0),
        }
    }
}
