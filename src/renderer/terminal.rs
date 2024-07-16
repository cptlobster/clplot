/// Direct interface for getting information about the terminal. Primarily used for getting the
/// screen size, and for learning about the capabilities of this terminal. (what colors does it
/// support? charsets?)
use crossterm::terminal::size;
pub fn get_size() -> (u16, u16) {
    size().expect("Error with terminal interaction")
}

pub fn get_width() -> u16 {
    get_size().0
}

pub fn get_height() -> u16 {
    get_size().1
}