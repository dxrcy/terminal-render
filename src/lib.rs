use std::thread;
use std::time::{Duration, Instant};

mod point;
pub mod shapes;

pub use point::Point;

pub trait Program {
    const FPS: u32;

    fn update(&mut self);

    fn render(&mut self);

    fn tick(&mut self) {
        timed(Duration::from_secs(1) / Self::FPS, || {
            self.update();
            self.render();
            terminal::flush();
        });
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

pub trait Drawable {
    fn draw(&self) {}
}

pub mod symbol {
    pub const LINE_HORIZONTAL: char = '─';
    pub const LINE_VERTICAL: char = '│';

    pub const LINE_DOWN_RIGHT: char = '┌';
    pub const LINE_DOWN_LEFT: char = '┐';
    pub const LINE_UP_RIGHT: char = '└';
    pub const LINE_UP_LEFT: char = '┘';

    // pub const ANGLE_CHARS: &[char]pub const ANGLE_CHARS: &[char] = &[ '|', '.', '/', '.', '-', '.', '\\', '.', '|', '.', '/', '.', '-', '.', '\\', '.', ];
    // pub const ANGLE_CHARS: &[char] = &['|', '/', '-', '\\', '|', '/', '-', '\\'];
    // pub const ANGLE_CHARS: &[char] = &['|', '·', '-', '·', '|', '·', '-', '·'];
    pub const ANGLE_CHARS: &[char] = &['|', '+', '-', '+', '|', '+', '-', '+'];
}

pub mod terminal {
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
