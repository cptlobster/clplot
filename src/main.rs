// clplot - Command line utility for plotting graphs and charts.
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
mod renderer;

use std::io::Result;
use crate::renderer::plot::{Plot, Point};
use crate::renderer::terminal::get_size;

fn main() -> Result<()> {
    let size: (u16, u16) = get_size();
    let width: u16 = size.0;
    let height: u16 = size.1;
    let plot: Plot = Plot::new(width, height);
    plot.clear();
    plot.put('b', Point::new(11, 5));
    plot.put('e', Point::new(20, 12));
    plot.put('a', Point::new(32, 7));
    plot.put('n', Point::new(45, 20));
    plot.put('s', Point::new(69, 22));
    plot.put('.', Point::new(80, 17));
    plot.put('.', Point::new(92, 25));
    plot.put('.', Point::new(110, 29));
    plot.put_str("ha! I love printing!", Point::new(1, 1));
    plot.put_str("what if I have...\na newline?", plot.origin_bl(3, 4));
    plot.put_str("AAAA\nAAAA\nAAAA\nAAAA", Point::new(1, 7));
    plot.put_str_transparent("B  B\nBB  \n  BB\n B B", Point::new(1, 7));
    plot.finish();
    Ok(())
}
