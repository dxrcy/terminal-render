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

        let rect = Rect::new(0, 0, 8, 12);
        rect.draw();

        let rect = Rect::new(3, 5, 4, 9);
        rect.draw();

        let circle = Circle::new(Point::new(40, 20), 10);
        circle.draw();

        // thread::sleep(Duration::from_millis(100));
    }
}

trait Drawable {
    fn draw(&self) {}
}

struct Circle {
    origin: Point<i32>,
    radius: u32,
}

impl Circle {
    pub fn new(origin: Point<i32>, radius: u32) -> Self {
        Self { origin, radius }
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        let r = self.radius as i32;
        let d = r * 2;
        let sx = self.origin.x - r;
        let sy = self.origin.y - r;

        for y in 0..d {
            terminal::cursor::move_to(sx, sy + y);

            for x in 0..d {
                let r2 = (x - r).pow(2) + (y - r).pow(2);
                if r2 < r.pow(2) as i32 {
                    print!("xx");
                } else {
                    print!("..");
                }
            }
            println!();
        }
    }
}

struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

struct SimpleLine {
    origin: Point<i32>,
    length: u32,
    orientation: Orientation,
}

enum Orientation {
    Horizontal,
    Vertical,
}

impl SimpleLine {
    pub fn new(origin: Point<i32>, length: u32, orientation: Orientation) -> Self {
        Self {
            origin,
            length,
            orientation,
        }
    }
}

impl Drawable for SimpleLine {
    fn draw(&self) {
        match self.orientation {
            Orientation::Horizontal => {
                for x in 0..(self.length as i32) {
                    terminal::cursor::move_to(self.origin.x + x * 2, self.origin.y);
                    print!("{}", symbol::LINE_HORIZONTAL);
                    print!("{}", symbol::LINE_HORIZONTAL);
                }
            }
            Orientation::Vertical => {
                for y in 0..(self.length as i32) {
                    terminal::cursor::move_to(self.origin.x, self.origin.y + y);
                    print!("{}", symbol::LINE_VERTICAL);
                }
            }
        }
    }
}

struct Rect {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

impl Drawable for Rect {
    fn draw(&self) {
        terminal::cursor::move_to(self.x, self.y);
        print!("{}", symbol::LINE_DOWN_RIGHT);
        print!("{}", symbol::LINE_HORIZONTAL);
        for _ in 0..self.w.saturating_sub(2) {
            print!("{}", symbol::LINE_HORIZONTAL);
            print!("{}", symbol::LINE_HORIZONTAL);
        }
        print!("{}", symbol::LINE_HORIZONTAL);
        print!("{}", symbol::LINE_DOWN_LEFT);

        for y in 1..self.h {
            terminal::cursor::move_to(self.x, self.y + y as i32);
            print!("{}", symbol::LINE_VERTICAL);
            terminal::cursor::move_right(self.w.saturating_sub(1) * 2);
            print!("{}", symbol::LINE_VERTICAL);
        }

        terminal::cursor::move_to(self.x, self.y + self.h as i32);
        print!("{}", symbol::LINE_UP_RIGHT);
        print!("{}", symbol::LINE_HORIZONTAL);
        for _ in 0..self.w.saturating_sub(2) {
            print!("{}", symbol::LINE_HORIZONTAL);
            print!("{}", symbol::LINE_HORIZONTAL);
        }
        print!("{}", symbol::LINE_HORIZONTAL);
        print!("{}", symbol::LINE_UP_LEFT);
    }
}

mod symbol {
    pub const LINE_HORIZONTAL: char = '─';
    pub const LINE_VERTICAL: char = '│';

    pub const LINE_DOWN_RIGHT: char = '┌';
    pub const LINE_DOWN_LEFT: char = '┐';
    pub const LINE_UP_RIGHT: char = '└';
    pub const LINE_UP_LEFT: char = '┘';
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

        pub fn move_right(n: u32) {
            print!("\x1b[{}C", n);
        }

        // TODO: Use `Point<u32>`
        pub fn move_to(x: i32, y: i32) {
            print!("\x1b[{};{}H", y + 1, x + 1);
        }
    }
}
