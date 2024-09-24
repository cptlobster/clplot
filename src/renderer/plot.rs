// clplot::renderer::plot - low-level API for drawing on command line
//     Copyright (C) 2024  Dustin Thomas <stdio@cptlobster.dev>
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.
/// Low level API for drawing on the command line. Has "plots" (2D area on the terminal that can be
/// drawn in by other utilities) and structures for basic shapes.
use std::cmp::{max, min};
use std::io::{Write, stdout, Stdout};
use crossterm::{cursor::{RestorePosition, SavePosition, MoveDown, MoveRight, MoveUp},
                queue, QueueableCommand, style::{Print}};
use tailcall::tailcall;
use crate::data::PVec2;

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
    /// Cut off the end of a string if it is too long.
    fn clip(input: &str, max_len: u16) -> &str {
        if (input.len() as u16) <= max_len { input } else { input.split_at(max_len as usize).0 }
    }

    /// Constrain a number within a range; If it falls outside the range, return the minimum or
    /// maximum value, depending on if it's smaller or larger.
    fn clamp(n: u16, lower: u16, upper: u16) -> u16 {
        max(lower, min(n, upper))
    }

    /// Constrain a point within a minimum and maximum XY range. Each dimension will be clamped
    /// separately.
    fn clamp_point(point: &PVec2, x_min: u16, x_max: u16, y_min: u16, y_max: u16) -> PVec2 {
        PVec2::new(Self::clamp(point.x, x_min, x_max), Self::clamp(point.y, y_min, y_max))
    }

    /// Constrain a point within the bounding box of this plot area.
    fn clamp_to_plot(&self, point: &PVec2) -> PVec2 {
        Self::clamp_point(point, self.x_min, self.x_max, self.y_min, self.y_max)
    }

    /// Derive a point from decimal (float) values (from 0.0 - 1.0). (0.0, 0.0) corresponds to top
    /// left, (1.0, 1.0) corresponds to bottom right.
    pub fn derive_point_dec(&self, x: f32, y: f32) -> PVec2 {
        let x_rd: u16 = x.round() as u16;
        let y_rd: u16 = y.round() as u16;
        self.clamp_to_plot(&PVec2::new(x_rd, y_rd))
    }

    /// Get a point offset from the top left of the plot area.
    pub fn origin_bl(&self, x: u16, y: u16) -> PVec2 {
        self.clamp_to_plot(&PVec2::new(x, self.height - y))
    }
    /// Get a point offset from the bottom right of the plot area.
    pub fn origin_br(&self, x: u16, y: u16) -> PVec2 {
        self.clamp_to_plot(&PVec2::new(self.width - x, self.height - y))
    }

    /// Create a new plot area of a specified width/height.
    pub fn new(width: u16, height: u16) -> Plot {
        let mut out: Stdout = stdout();
        let nls: String = "\n".repeat(height as usize);
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
        let nls: String = "\n".repeat(height as usize);
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
        let cleared_area: String = (" ".repeat(self.width as usize) + "\n").repeat(self.height as usize);
        queue!(out, RestorePosition, MoveUp(self.height), Print(cleared_area));
        out.flush().expect("Error with terminal interaction");
    }

    /// Place a character at a location on the plot area.
    pub fn put(&self, character: char, point: &PVec2) {
        let actual : PVec2 = self.clamp_to_plot(point);
        let mut out: Stdout = stdout();
        queue!(out, RestorePosition, MoveUp(self.height - actual.y), MoveRight(actual.x), Print(character));
        out.flush().expect("Error with terminal interaction");
    }

    /// Print a string on the plot area. Note that whitespace will overwrite existing content; You
    /// can use `put_str_transparent()` instead if you want to ignore whitespace.
    pub fn put_str(&self, content: &str, start: &PVec2) {
        let mut out: Stdout = stdout();
        let actual : PVec2 = self.clamp_to_plot(start);
        queue!(out, RestorePosition, MoveUp(self.height - actual.y), MoveRight(actual.x));
        let lines = content.split("\n");
        for line in lines {
            queue!(out, Print(Self::clip(line, self.width - actual.x)), Print("\n"), MoveRight(actual.x));
        }
        out.flush().expect("Error with terminal interaction");
    }

    /// Helper function for `put_str_transparent()`.
    #[tailcall]
    fn consume_line(out: &mut Stdout, line: &str) {
        if line.len() == 0 { return }
        let Some((left, right)) = line.find(|a: char| { a.is_whitespace() }).map(|i| line.split_at(i)) else {
            out.queue(Print(line)).expect("Error with terminal interaction");
            return
        };
        out.queue(Print(left)).expect("Error with terminal interaction");
        let Some((l2, r2)) = right.find(|a: char| { !a.is_whitespace() }).map(|i| right.split_at(i)) else {
            return
        };
        out.queue(MoveRight(l2.len() as u16)).expect("Error with terminal interaction");
        Self::consume_line(out, r2)
    }

    /// Put a string on the plot area. Whitespace will not overwrite existing content.
    pub fn put_str_transparent(&self, content: &str, start: &PVec2) {
        let mut out: Stdout = stdout();
        let actual : PVec2 = self.clamp_to_plot(start);
        queue!(out, RestorePosition, MoveUp(self.height - actual.y), MoveRight(actual.x));
        let lines = content.split("\n");
        for line in lines {
            Self::consume_line(&mut out, Self::clip(line, self.width - actual.x));
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