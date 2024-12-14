use termrender::shapes::{Circle, Rect};
use termrender::terminal;
use termrender::Point;
use termrender::{Drawable as _, Program};

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

impl Program for App {
    const FPS: u32 = 2;

    fn update(&mut self) {
        self.frame_number += 1;
    }

    fn render(&mut self) {
        terminal::clear();

        let rect = Rect::new(0, 0, 8, 12);
        rect.draw();

        let rect = Rect::new(3, 5, 4, 9);
        rect.draw();

        let origin = Point::new(40, 20);
        let radius = self.frame_number as i32 % 10 + 10;

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
