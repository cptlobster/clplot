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