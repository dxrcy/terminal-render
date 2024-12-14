use std::f32::consts::PI;

use crate::{symbol, terminal, Drawable, Point};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
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

#[derive(Clone, Copy, Debug)]
pub struct Circle {
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
        if self.radius < 2 {
            terminal::cursor::move_to(self.origin);
            print!(
                "{}",
                if self.radius < 1 {
                    symbol::SMALLEST_CIRCLE
                } else {
                    symbol::SMALL_CIRCLE
                }
            );
            return;
        }

        let pixel = |point: Point<i32>| {
            let coord: Point<f32> = (point - self.origin).into();
            let angle = (coord.y).atan2(coord.x / 2.0);
            let ch = get_angle_character(angle);

            terminal::cursor::move_to(point);
            print!("{}", ch);
        };

        let ry = self.radius as f32 - 1.0;
        midpoint_ellipse(self.origin, ry * 2.0, ry, pixel);
        // panic!();
    }
}

fn get_angle_character(angle: f32) -> char {
    use symbol::ANGLE_CHARS;

    /// Adjust angle to align properly
    const OFFSET: f32 = 1.0 + 1.0 / ANGLE_CHARS.len() as f32;

    // Normalize angle to [0, 2*pi)
    let adjusted = (angle + PI * OFFSET) % (PI * 2.0);

    let index = adjusted / PI / 2.0 * ANGLE_CHARS.len() as f32;
    ANGLE_CHARS[index as usize]
}

fn midpoint_ellipse<F>(origin: Point<i32>, rx: f32, ry: f32, pixel: F)
where
    F: Fn(Point<i32>) -> (),
{
    let pixels = |x: f32, y: f32| {
        [(1, 1), (1, -1), (-1, 1), (-1, -1)]
            .iter()
            .for_each(|(sx, sy)| {
                pixel(origin + (x as i32 * sx, y as i32 * sy));
            })
    };

    let mut x = 0.0;
    let mut y = ry;

    let mut dx = 0.0;
    let mut dy = 2.0 * ry * rx.powi(2);

    let mut d1 = ry.powi(2) + rx.powi(2) * (-ry + 0.25);

    while dx < dy {
        pixels(x, y);
        x += 1.0;

        if d1 < 0.0 {
            dx += 2.0 * ry.powi(2);
            d1 += dx + 2.0 * ry.powi(2);
        } else {
            y -= 1.0;
            dx += 2.0 * ry.powi(2);
            dy -= 2.0 * rx.powi(2);
            d1 += dx - dy + ry.powi(2);
        }
    }

    let mut d2 =
        ry.powi(2) * (x + 0.5).powi(2) + rx.powi(2) * (y - 1.0).powi(2) - (rx * ry).powi(2);

    while y >= 0.0 {
        pixels(x, y);
        y -= 1.0;

        if d2 > 0.0 {
            dy -= 2.0 * rx.powi(2);
            d2 += rx.powi(2) - dy;
        } else {
            x += 1.0;
            dx += 2.0 * ry.powi(2);
            dy -= 2.0 * rx.powi(2);
            d2 += dx - dy + rx.powi(2);
        }
    }
}
