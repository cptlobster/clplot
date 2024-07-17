/// Basic shapes.
use crate::PVec2;
use crate::renderer::plot::Plot;

/// Basic structure for representing a 2D value (position, size, etc.). These are represented as
/// floating point values to allow flexibility for input data.
#[derive(PartialEq, Copy, Clone)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 { Vec2 { x, y } }

    /// Determine what values to add to reach an other point.
    ///
    /// ## Example
    /// ```rs
    /// let a: Point = Point::new(1, 3)
    /// let b: Point = Point::new(2, 4)
    /// let c: Point = a.to(b) // returns Point(1, 1)
    /// let d: bool = a + c == b // returns true
    /// ```
    pub fn to(&self, other: &Vec2) -> Vec2 { Vec2::new(other.x - self.x, other.y - self.y) }

    /// Get the distance between two points.
    pub fn distance(&self, other: &Vec2) -> f32 {
        let dist = self.to(other);
        (dist.x * dist.x) + (dist.y * dist.y).sqrt()
    }
}

/// The "view box" provides an easy way to handle multiple things:
/// - It can constrain shapes to a specific portion of the plot area
/// - It allows for converting from arbitrary scales to plot coordinate values.
pub struct ViewBox {
    position: PVec2,
    size: PVec2,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

impl ViewBox {
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
    pub fn in_viewbox(viewbox: ViewBox, position: Vec2, symbol: char) -> Point {
        Self::new(viewbox.translate_to_plot(position), symbol)
    }
    pub fn draw(&self, plot: &Plot) {
        plot.put(self.symbol, &self.position);
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
    pub fn in_viewbox(viewbox: ViewBox, start: Vec2, end: Vec2, symbol: char) -> Line {
        Line::new(viewbox.translate_to_plot(start), viewbox.translate_to_plot(end), symbol)
    }
    pub fn draw(&self, plot: &Plot) {
        let dx: i16 = self.end.x as i16 - self.start.x as i16;
        let dy: i16 = self.end.y as i16 - self.start.y as i16;
        // if this is a straight line on either the X-axis or the Y-axis, make this easy
        if dy == 0 {
            let line: &str = self.symbol.to_string().repeat(dx.abs() as usize).as_str();
            plot.put_str(line, &PVec2::new(self.start.x.min(self.end.x), self.start.y))
        }
        else if dx == 0 {
            let line: &str = "\n".repeat(dy.abs() as usize).as_str();
            plot.put_str(line, &PVec2::new(self.start.x, self.start.y.min(self.end.y)))
        }
        else {
            // make the string the hard way
            // TODO: implement
        }
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
    pub fn in_viewbox(viewbox: ViewBox, position: Vec2, size: Vec2, symbol: char) -> Rect {
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
}