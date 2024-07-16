/// This module contains components for using the command line as a display for complex graphics.
/// The renderer API directly manipulates stdout to draw shapes, and utilizes ANSI escape sequences
/// to handle drawing in arbitrary locations and colors.
pub mod terminal;
pub mod plot;