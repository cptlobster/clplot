/// Low level API for drawing on the command line. Has "plots" (2D area on the terminal that can be
/// drawn in by other utilities) and structures for basic shapes.
use std::cmp::{max, min};
use std::io::{self};

use crossterm::{cursor::{RestorePosition, SavePosition},
                ExecutableCommand};
use crossterm::cursor::{MoveDown, MoveRight, MoveUp};

/// Basic structure for representing a 2D position on a plot. Since plots use only unsigned integer
/// values, this struct only supports unsigned integers.
#[derive(PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point { Point { x, y } }

    /// Determine what values to add to reach an other point.
    ///
    /// ## Example
    /// ```rs
    /// let a: Point = Point::new(1, 3)
    /// let b: Point = Point::new(2, 4)
    /// let c: Point = a.to(b) // returns Point(1, 1)
    /// let d: bool = a + c == b // returns true
    /// ```
    pub fn to(&self, other: &Point) -> Point { Point::new(other.x - self.x, other.y - self.y) }
}

/// Basic plot object.
pub struct Plot {
    pub width: u16,
    pub height: u16,
    pub x_min: u16,
    pub x_max: u16,
    pub y_min: u16,
    pub y_max: u16,
}

impl Plot {
    fn clamp(n: u16, lower: u16, upper: u16) -> u16 {
        max(lower, min(n, upper))
    }

    fn clamp_point(point: Point, x_min: u16, x_max: u16, y_min: u16, y_max: u16) -> Point {
        Point::new(Self::clamp(point.x, x_min, x_max), Self::clamp(point.y, y_min, y_max))
    }

    fn clamp_to_plot(&self, point: Point) -> Point {
        Self::clamp_point(point, self.x_min, self.x_max, self.y_min, self.y_max)
    }

    pub fn derive_point_dec(&self, x: f32, y: f32) -> Point {
        let x_rd: u16 = x.round() as u16;
        let y_rd: u16 = y.round() as u16;
        self.clamp_to_plot(Point::new(x_rd, y_rd))
    }

    /// Create a new plot area of a specified width/height.
    pub fn new(width: u16, height: u16) -> Plot {
        let nls: String = (0..height).map(|_| '\n').collect::<String>();
        print!("{}", nls);
        io::stdout()
            .execute(SavePosition)
            .expect("Error with terminal interaction");
        Plot {
            width,
            height,
            x_min: 0,
            x_max: width,
            y_min: 0,
            y_max: height,
        }
    }

    /// Resize plot to new width/height. It is recommended that you clear the plot after you
    /// change size.
    pub fn resize(&self, width: u16, height: u16) -> Plot {
        io::stdout()
            .execute(RestorePosition)
            .expect("Error with terminal interaction");
        io::stdout()
            .execute(MoveUp(self.height))
            .expect("Error with terminal interaction");
        let nls: String = (0..height).map(|_| '\n').collect::<String>();
        print!("{}", nls);
        Plot {
            width,
            height,
            x_min: 0,
            x_max: width,
            y_min: 0,
            y_max: height,
        }
    }

    /// Clear the plot area (fill the entire area with spaces).
    pub fn clear(&self) {
        io::stdout()
            .execute(RestorePosition)
            .expect("Error with terminal interaction");
        io::stdout()
            .execute(MoveUp(self.height))
            .expect("Error with terminal interaction");
        let cleared_area: String = (0..self.height).map(
            |_| (0..self.width).map(|_| " ").collect::<String>() + "\n")
            .collect::<String>();
        print!("{}", cleared_area)
    }

    /// Place a character at a location on the plot area.
    pub fn put(&self, character: char, point: Point) {
        let actual : Point = self.clamp_to_plot(point);
        io::stdout()
            .execute(RestorePosition)
            .expect("Error with terminal interaction");
        io::stdout()
            .execute(MoveUp(self.height - actual.y))
            .expect("Error with terminal interaction");
        io::stdout()
            .execute(MoveRight(actual.x))
            .expect("Error with terminal interaction");
        print!("{}", character);
    }
    /// Run this when you are done with the plot; This will position the cursor on the line below,
    /// so that the plot remains visible.
    pub fn finish(&self) {
        io::stdout()
            .execute(RestorePosition)
            .expect("Error with terminal interaction");
        io::stdout()
            .execute(MoveDown(1))
            .expect("Error with terminal interaction");
    }
}