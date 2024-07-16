/// Low level API for drawing on the command line. Has "plots" (2D area on the terminal that can be
/// drawn in by other utilities) and structures for basic shapes.
use std::cmp::{max, min};
use std::io::{Write, stdout, Stdout};

use crossterm::{cursor::{RestorePosition, SavePosition, MoveDown, MoveRight, MoveUp},
                queue, QueueableCommand, style::{Print}};
use tailcall::tailcall;

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

    /// Derive a point from decimal (float) values (from 0.0 - 1.0). (0.0, 0.0) corresponds to top
    /// left, (1.0, 1.0) corresponds to bottom right.
    pub fn derive_point_dec(&self, x: f32, y: f32) -> Point {
        let x_rd: u16 = x.round() as u16;
        let y_rd: u16 = y.round() as u16;
        self.clamp_to_plot(Point::new(x_rd, y_rd))
    }

    /// Get a point offset from the top left of the plot area.
    pub fn origin_bl(&self, x: u16, y: u16) -> Point {
        self.clamp_to_plot(Point::new(x, self.height - y))
    }
    /// Get a point offset from the bottom right of the plot area.
    pub fn origin_br(&self, x: u16, y: u16) -> Point {
        self.clamp_to_plot(Point::new(self.width - x, self.height - y))
    }

    /// Create a new plot area of a specified width/height.
    pub fn new(width: u16, height: u16) -> Plot {
        let mut out: Stdout = stdout();
        let nls: String = (0..height).map(|_| '\n').collect::<String>();
        queue!(out, Print(nls), SavePosition);
        out.flush().expect("Error with terminal interaction");
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
        let mut out: Stdout = stdout();
        let nls: String = (0..height).map(|_| '\n').collect::<String>();
        queue!(out, RestorePosition, MoveUp(self.height), Print(nls), SavePosition);
        out.flush().expect("Error with terminal interaction");
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
        let mut out: Stdout = stdout();
        let cleared_area: String = (0..self.height).map(
            |_| (0..self.width).map(|_| " ").collect::<String>() + "\n")
            .collect::<String>();
        queue!(out, RestorePosition, MoveUp(self.height), Print(cleared_area));
        out.flush().expect("Error with terminal interaction");
    }

    /// Place a character at a location on the plot area.
    pub fn put(&self, character: char, point: Point) {
        let actual : Point = self.clamp_to_plot(point);
        let mut out: Stdout = stdout();
        queue!(out, RestorePosition, MoveUp(self.height - actual.y), MoveRight(actual.x), Print(character));
        out.flush().expect("Error with terminal interaction");
    }

    /// Print a string on the plot area. Note that whitespace will overwrite existing content; You
    /// can use `put_str_transparent()` instead if you want to ignore whitespace.
    pub fn put_str(&self, content: &str, start: Point) {
        let mut out: Stdout = stdout();
        let actual : Point = self.clamp_to_plot(start);
        queue!(out, RestorePosition, MoveUp(self.height - actual.y), MoveRight(actual.x));
        let lines = content.split("\n");
        for line in lines {
            queue!(out, Print(line), Print("\n"), MoveRight(actual.x));
        }
        out.flush().expect("Error with terminal interaction");
    }

    #[tailcall]
    fn consume_line(out: &mut Stdout, line: &str) {
        if line.len() == 0 { return }
        let Some((left, right)) = line.find(|a: char| { a.is_whitespace() }).map(|i| line.split_at(i)) else {
            out.queue(Print(line)).expect("Error with terminal interaction");
            out.flush().expect("Error with terminal interaction");
            return
        };
        out.queue(Print(left)).expect("Error with terminal interaction");
        out.flush().expect("Error with terminal interaction");
        let Some((l2, r2)) = right.find(|a: char| { !a.is_whitespace() }).map(|i| right.split_at(i)) else {
            return
        };
        out.queue(MoveRight(l2.len() as u16)).expect("Error with terminal interaction");
        out.flush().expect("Error with terminal interaction");
        Self::consume_line(out, r2)
    }

    /// Put a string on the plot area. Whitespace will not overwrite existing content.
    pub fn put_str_transparent(&self, content: &str, start: Point) {
        let mut out: Stdout = stdout();
        let actual : Point = self.clamp_to_plot(start);
        queue!(out, RestorePosition, MoveUp(self.height - actual.y), MoveRight(actual.x));
        let lines = content.split("\n");
        for line in lines {
            Self::consume_line(&mut out, line);
            queue!(out, Print("\n"), MoveRight(actual.x));
        }
        out.flush().expect("Error with terminal interaction");
    }

    /// Run this when you are done with the plot; This will position the cursor on the line below,
    /// so that the plot remains visible.
    pub fn finish(&self) {
        let mut out: Stdout = stdout();
        queue!(out, RestorePosition, MoveDown(1));
        out.flush().expect("Error with terminal interaction");
    }
}