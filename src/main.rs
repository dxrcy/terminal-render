use std::thread;
use std::time::{Duration, Instant};

const FPS: u32 = 2;

fn main() {
    let mut app = App::new(10, 10);

    loop {
        app.tick();
    }
}

struct App {
    width: u32,
    height: u32,
    circles: Vec<Circle>,
}

struct Circle {
    origin: Point,
    radius: f32,
}

struct Point {
    x: f32,
    y: f32,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            circles: Vec::new(),
        }
    }
}

impl App {
    pub fn tick(&mut self) {
        let then = Instant::now();

        self.render();
        terminal::flush();

        let delta = Instant::now() - then;
        let duration = (Duration::from_secs(1) / FPS).saturating_sub(delta);
        thread::sleep(duration);
    }

    fn render(&mut self) {
        terminal::clear();

        let rect = Rect::new(0.0, 0.0, self.width as f32, self.height as f32);
        rect.draw();

        // thread::sleep(Duration::from_millis(100));
    }
}

trait Drawable {
    fn draw(&self) {}
}

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

impl Drawable for Rect {
    fn draw(&self) {
        for y in 0..(self.h as u32) {
            terminal::cursor::move_to(self.x as u32 + 1, self.y as u32 + y + 1);

            for _ in 0..(self.w as u32) {
                print!("x");
            }
        }
    }
}

mod terminal {
    use std::io::{self, Write as _};

    pub fn flush() {
        io::stdout().flush().unwrap();
    }

    pub fn clear() {
        print!("\x1b[2J");
        print!("\x1b[0;0H");
    }

    pub mod cursor {

        pub fn move_down(n: u32) {
            print!("\x1b[{}B", n);
        }

        pub fn move_to(x: u32, y: u32) {
            print!("\x1b[{};{}H", y, x);
        }
    }
}
