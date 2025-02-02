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
/// Common shapes and drawing code.

/// Basic shapes.
use crate::renderer::plot::Plot;
use crate::data::{Vec2, PVec2};

/// The "view box" provides an easy way to constrain shapes to a specific portion of the plot area.
pub struct ViewBox {
    plot: Plot,
    position: PVec2,
    size: PVec2,
}

impl ViewBox {
    fn clamp(n: u16, lower: u16, upper: u16) -> u16 {
        lower.max(n.min(upper))
    }
    
    fn clamp_point(point: PVec2, x_min: u16, x_max: u16, y_min: u16, y_max: u16) -> PVec2 {
        PVec2::new(Self::clamp(point.x, x_min, x_max), Self::clamp(point.y, y_min, y_max))
    }

    fn clamp_to_plot(&self, point: PVec2) -> PVec2 {
        Self::clamp_point(point, 0, self.size.x, 0, self.size.y)
    }

    /// Translates relative coordinates to absolute coordinates on the parent plot
    pub fn translate_to_plot(&self, point: PVec2) -> PVec2 {
        self.clamp_to_plot(point + self.position)
    }
}

/// The "scaled view box" provides an easy way to handle multiple things:
/// - It can constrain shapes to a specific portion of the plot area
/// - It allows for converting from arbitrary scales to plot coordinate values.
pub struct ScaledViewBox {
    plot: Plot,
    position: PVec2,
    size: PVec2,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

impl ScaledViewBox {
    fn clamp(n: f32, lower: f32, upper: f32) -> f32 {
        lower.max(n.min(upper))
    }
    fn clamp_point(point: Vec2, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Vec2 {
        Vec2::new(Self::clamp(point.x, x_min, x_max), Self::clamp(point.y, y_min, y_max))
    }

    fn clamp_to_plot(&self, point: Vec2) -> Vec2 {
        Self::clamp_point(point, self.x_min, self.x_max, self.y_min, self.y_max)
    }

    fn scale_to_dec(&self, point: Vec2) -> Vec2 {
        Vec2::new(
            (point.x - self.x_min) / self.x_max - self.x_min,
            (point.y - self.y_min) / self.y_max - self.y_min,
        )
    }

    fn dec_to_pp(&self, point: Vec2) -> PVec2 {
        PVec2::new(
            (point.x * self.size.x as f32) as u16 + self.position.x,
            (point.y * self.size.y as f32) as u16 + self.position.y,
        )
    }

    /// Translates floating-point values (defined by the bounds on the viewbox itself) into plot
    /// area coordinates.
    pub fn translate_to_plot(&self, point: Vec2) -> PVec2 {
        self.dec_to_pp(self.scale_to_dec(self.clamp_to_plot(point)))
    }
}

/// A point. Can be drawn on a plot area.
pub struct Point {
    position: PVec2,
    symbol: char,
}

impl Point {
    pub fn new(position: PVec2, symbol: char) -> Point {
        Point {position, symbol}
    }
    /// Create a point based on a ScaledViewBox's coordinate system and convert it to integer coordinates.
    pub fn in_svb(viewbox: ScaledViewBox, position: Vec2, symbol: char) -> Point {
        Self::new(viewbox.translate_to_plot(position), symbol)
    }
    /// Draw the point in the selected plot area.
    pub fn draw(&self, plot: &Plot) {
        plot.put(self.symbol, &self.position);
    }
    /// Draw the point in the selected ViewBox. This will translate to the ViewBox's origin.
    pub fn draw_vb(&self, viewbox: &ViewBox) {
        Self::new(self.position + viewbox.position, self.symbol).draw(&viewbox.plot)
    }
}

/// A line. Can be drawn on a plot area.
pub struct Line {
    start: PVec2,
    end: PVec2,
    symbol: char,
}

impl Line {
    pub fn new(start: PVec2, end: PVec2, symbol: char) -> Line {
        Line { start, end, symbol }
    }
    pub fn in_svb(viewbox: ScaledViewBox, start: Vec2, end: Vec2, symbol: char) -> Line {
        Line::new(viewbox.translate_to_plot(start), viewbox.translate_to_plot(end), symbol)
    }
    pub fn draw(&self, plot: &Plot) {
        let dx: i16 = self.end.x as i16 - self.start.x as i16;
        let dy: i16 = self.end.y as i16 - self.start.y as i16;
        // if this is a straight line on either the X-axis or the Y-axis, make this easy
        if dy == 0 {
            let line: String = self.symbol.to_string().repeat(dx.abs() as usize);
            plot.put_str(line.as_str(), &PVec2::new(self.start.x.min(self.end.x), self.start.y))
        }
        else if dx == 0 {
            let line: String = (self.symbol.to_string() + "\n").repeat(dy.abs() as usize);
            plot.put_str(line.as_str(), &PVec2::new(self.start.x, self.start.y.min(self.end.y)))
        }
        // make the string the hard way; if you somehow make it to this with either dx or dy = 0, I
        // would be very concerned and would expect this to fail spectacularly. Good luck, friend.
        else {
            // determine our step size on the x axis
            // we scale our step value so that the y step is 1; this allows us to generate our line,
            // line by line
            let mut step_x: f32 = dx as f32 / dy as f32;
            // create position and target values
            let mut px: f32 = self.start.x as f32;
            let mut tx: f32 = self.end.x as f32;
            if (step_x < 0.0) {
                px = self.end.x as f32;
                tx = self.start.x as f32;
            }
            let mut py: u16 = self.start.y.min(self.end.y);
            let ty: u16 = self.end.y.max(self.start.y);
            let mut lines: String = "".to_string();
            // create the string for the line
            // this is probably horribly inefficient, I should really figure out a way to make this
            // run better. it works for now at least.
            while (px != tx && py != ty) {
                let prev_x: f32 = px;
                px += step_x;
                // get start and end points for the actual line segment
                let str_start: i16 = px.min(prev_x).floor() as i16;
                let str_end: i16 = px.max(prev_x).ceil() as i16;
                // get the length of the line segment
                let str_len: i16 = str_end - str_start;
                // fill from 0 to start with whitespace, start to end with character
                let line: String = ' '.to_string().repeat(str_start as usize) + self.symbol.to_string().repeat(str_len as usize).as_str();
                // finish it off with a newline
                lines += (line + "\n").as_str();
                py += 1
            }
            // push this god-awful monstrosity to the plot
            plot.put_str_transparent(lines.as_str(), &PVec2::new(self.start.x, self.start.y.min(self.end.y)));
        }
    }
    pub fn draw_vb(&self, viewbox: &ViewBox) {
        Self::new(self.start + viewbox.position, self.end + viewbox.position, self.symbol).draw(&viewbox.plot)
    }
}

/// A rectangle. Can be drawn on a plot area.
pub struct Rect {
    position: PVec2,
    size: PVec2,
    symbol: char,
}

impl Rect {
    pub fn new(position: PVec2, size: PVec2, symbol: char) -> Rect {
        Rect { position, size, symbol }
    }

    pub fn in_svb(viewbox: ScaledViewBox, position: Vec2, size: Vec2, symbol: char) -> Rect {
        Rect::new(viewbox.translate_to_plot(position), viewbox.translate_to_plot(size), symbol)
    }

    pub fn draw(&self, plot: &Plot) {
        let tl: PVec2 = self.position;
        let tr: PVec2 = PVec2::new(self.position.x + self.size.x, self.position.y);
        let bl: PVec2 = PVec2::new(self.position.x, self.position.y + self.size.y);
        let br: PVec2 = PVec2::new(self.position.x + self.size.x, self.position.y + self.size.y);

        Line::new(tl, tr, self.symbol).draw(plot);
        Line::new(bl, br, self.symbol).draw(plot);
        Line::new(tl, bl, self.symbol).draw(plot);
        Line::new(tr, br, self.symbol).draw(plot);
    }
    pub fn draw_vb(&self, viewbox: &ViewBox) {
        Self::new(self.position + viewbox.position, self.size, self.symbol).draw(&viewbox.plot)
    }
}