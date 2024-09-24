use std::ops::{Add, Sub};

/// Basic structure for representing a 2D position on a plot. Since plots use only unsigned integer
/// values, this struct only supports unsigned integers.
#[derive(PartialEq, Copy, Clone)]
pub struct PVec2 {
    pub x: u16,
    pub y: u16,
}

impl PVec2 {
    pub fn new(x: u16, y: u16) -> PVec2 { PVec2 { x, y } }

    /// Determine what values to add to reach another point.
    ///
    /// ## Example
    /// ```rs
    /// let a: Point = Point::new(1, 3)
    /// let b: Point = Point::new(2, 4)
    /// let c: Point = a.to(b) // returns Point(1, 1)
    /// let d: bool = a + c == b // returns true
    /// ```
    pub fn to(&self, other: &PVec2) -> PVec2 { PVec2::new(other.x - self.x, other.y - self.y) }

    fn distance(self: &PVec2, rhs: &PVec2) -> f32 {
        let lx: f32 = self.x as f32;
        let ly: f32 = self.y as f32;
        let rx: f32 = rhs.x as f32;
        let ry: f32 = rhs.y as f32;
        ((lx - rx).powi(2) + (ly - ry).powi(2)).sqrt()
    }
}

impl Add for PVec2 {
    type Output = PVec2;

    fn add(self, rhs: Self) -> Self::Output {
        PVec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for PVec2 {
    type Output = PVec2;

    fn sub(self, rhs: Self) -> Self::Output {
        PVec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// Basic structure for representing 2D points on any arbitrary coordinate plane. Uses floats to
/// allow for decimal values, and can be overlaid onto a ScaledViewBox to get proper coordinates.
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

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}