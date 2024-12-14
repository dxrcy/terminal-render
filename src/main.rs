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
    frame_number: usize,
    width: u32,
    height: u32,
    circles: Vec<Circle>,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            frame_number: 0,
            width,
            height,
            circles: Vec::new(),
        }
    }
}

impl App {
    pub fn tick(&mut self) {
        timed(Duration::from_secs(1) / FPS, || {
            self.frame_number += 1;
            self.render();
            terminal::flush();
        });
    }

    fn render(&mut self) {
        terminal::clear();

        let rect = Rect::new(0, 0, 8, 12);
        rect.draw();

        let rect = Rect::new(3, 5, 4, 9);
        rect.draw();

        let origin = Point::new(40, 20);
        let radius = self.frame_number as i32 % 20;

        // let rect = Rect::new(
        //     origin.x - radius * 2 - 1,
        //     origin.y - radius - 1,
        //     radius as u32 * 4 + 2,
        //     radius as u32 * 2 + 2,
        // );
        // rect.draw();

        let circle = Circle::new(origin, radius as u32);
        circle.draw();

        // thread::sleep(Duration::from_millis(100));
    }
}

// TODO: Rename
fn timed<F>(duration: Duration, func: F)
where
    F: FnOnce() -> (),
{
    let then = Instant::now();
    func();
    let delta = Instant::now() - then;
    thread::sleep(duration.saturating_sub(delta));
}

trait Drawable {
    fn draw(&self) {}
}

#[derive(Clone, Copy, Debug)]
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
        let sx = self.origin.x - r * 2;
        let sy = self.origin.y - r;

        // Discusting hack!
        let pred = |x: i32, y: i32| {
            let y = y as f32 + 0.0;
            let x = x as f32 / 2.0 + 0.0;
            let r = r as f32;
            let r2 = (x - r).powi(2) + (y - r).powi(2);
            r2 < (r + 0.5).powi(2) && r2 > (r - 0.5).powi(2)
        };

        for y in 0..d + 1 {
            terminal::cursor::move_to((sx, sy + y));

            for x in 0..d * 2 + 1 {
                if pred(x, y) {
                    print!(".");
                } else {
                    terminal::cursor::move_right(1);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T, U> From<(T, T)> for Point<U>
where
    Point<U>: From<Point<T>>,
    U: From<T>,
{
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    // TODO: Use `Point<i32>`
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
        terminal::cursor::move_to((self.x, self.y));
        print!("{}", symbol::LINE_DOWN_RIGHT);
        // print!("{}", symbol::LINE_HORIZONTAL);
        for _ in 0..self.w.saturating_sub(1) {
            print!("{}", symbol::LINE_HORIZONTAL);
            // print!("{}", symbol::LINE_HORIZONTAL);
        }
        // print!("{}", symbol::LINE_HORIZONTAL);
        print!("{}", symbol::LINE_DOWN_LEFT);

        for y in 1..self.h {
            terminal::cursor::move_to((self.x, self.y + y as i32));
            print!("{}", symbol::LINE_VERTICAL);
            terminal::cursor::move_right(self.w.saturating_sub(1) * 1);
            print!("{}", symbol::LINE_VERTICAL);
        }

        terminal::cursor::move_to((self.x, self.y + self.h as i32));
        print!("{}", symbol::LINE_UP_RIGHT);
        // print!("{}", symbol::LINE_HORIZONTAL);
        for _ in 0..self.w.saturating_sub(1) {
            print!("{}", symbol::LINE_HORIZONTAL);
            // print!("{}", symbol::LINE_HORIZONTAL);
        }
        // print!("{}", symbol::LINE_HORIZONTAL);
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
        use crate::Point;

        pub fn move_down(n: u32) {
            print!("\x1b[{}B", n);
        }

        pub fn move_right(n: u32) {
            print!("\x1b[{}C", n);
        }

        pub fn move_to(point: impl Into<Point<i32>>) {
            let point = point.into();
            let x = point.x.max(0) as u32;
            let y = point.y.max(0) as u32;
            print!("\x1b[{};{}H", y + 1, x + 1);
        }
    }
}

// #[derive(Clone, Copy, Debug)]
// struct SimpleLine {
//     origin: Point<i32>,
//     length: u32,
//     orientation: Orientation,
// }
//
// #[derive(Clone, Copy, Debug)]
// enum Orientation {
//     Horizontal,
//     Vertical,
// }
//
// impl SimpleLine {
//     pub fn new(origin: Point<i32>, length: u32, orientation: Orientation) -> Self {
//         Self {
//             origin,
//             length,
//             orientation,
//         }
//     }
// }
//
// impl Drawable for SimpleLine {
//     fn draw(&self) {
//         match self.orientation {
//             Orientation::Horizontal => {
//                 for x in 0..(self.length as i32) {
//                     terminal::cursor::move_to((self.origin.x + x * 2, self.origin.y));
//                     print!("{}", symbol::LINE_HORIZONTAL);
//                     print!("{}", symbol::LINE_HORIZONTAL);
//                 }
//             }
//             Orientation::Vertical => {
//                 for y in 0..(self.length as i32) {
//                     terminal::cursor::move_to((self.origin.x, self.origin.y + y));
//                     print!("{}", symbol::LINE_VERTICAL);
//                 }
//             }
//         }
//     }
// }
