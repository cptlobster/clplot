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
mod chart;
mod data;

use std::io::Result;
use crate::renderer::plot::Plot;
use crate::data::PVec2;
use crate::renderer::shapes::Line;
use crate::renderer::terminal::get_size;
use clap::{Parser, Subcommand};
use clio::Input;

/// Command-line graphing and plotting utility.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Input data. Defaults to stdin.
    #[arg(short, long, value_parser, default_value="-")]
    input_file: Input,

    /// Format of data input.
    #[arg(short, long, default_value="float")]
    format: String,

    /// Width of draw area. If undefined, uses full width of console.
    #[arg(short='W', long, default_value_t = 0)]
    width: u16,

    /// Height of draw area. If undefined, uses full height of console.
    #[arg(short='H', long, default_value_t = 0)]
    height: u16,
}

#[derive(Subcommand)]
enum Commands {
    /// Display test output
    Test {},
}

fn main() -> Result<()> {
    let args = Args::parse();

    // set plot width and height
    let size: (u16, u16) = get_size();
    let width: u16;
    if (args.width != 0) { width = args.width; }
    else { width = size.0; }
    let height: u16;
    if (args.height != 0) { height = args.height; }
    else { height = size.1 - 1; }

    // create and clear the plot area
    let plot: Plot = Plot::new(width, height);
    plot.clear();

    match &args.command {
        Commands::Test{} => {
            // print all our test characters
            plot.put('b', &PVec2::new(11, 5));
            plot.put('e', &PVec2::new(20, 12));
            plot.put('a', &PVec2::new(32, 7));
            plot.put('n', &PVec2::new(45, 20));
            plot.put('s', &PVec2::new(69, 22));
            plot.put('.', &PVec2::new(80, 17));
            plot.put('.', &PVec2::new(92, 25));
            plot.put('.', &PVec2::new(110, 29));
            plot.put_str("ha! I love printing!", &PVec2::new(3, 1));
            plot.put_str("what if I have...\na newline?", &plot.origin_bl(3, 4));
            plot.put_str("AAAA\nAAAA\nAAAA\nAAAA", &PVec2::new(3, 7));
            plot.put_str_transparent("B  B\nBB  \n  BB\n B B", &PVec2::new(3, 7));
            let l1 = Line::new(PVec2::new(1, 1), plot.origin_bl(1, 1), '|');
            let l2 = Line::new(plot.origin_bl(1, 1), plot.origin_br(1, 1), '-');
            l1.draw(&plot);
            l2.draw(&plot);
            plot.clear();
            plot.put_str("this should be different...", &PVec2::new(3, 1));
            plot.put_str("what if I have...\na newline?", &plot.origin_bl(3, 4));
            plot.put_str("AAAA\nAAAA\nAAAA\nAAAA", &PVec2::new(3, 7));
            plot.put_str_transparent("B  B\nBB  \n  BB\n B B", &PVec2::new(3, 7));
            let l1 = Line::new(PVec2::new(1, 1), plot.origin_bl(1, 1), '|');
            let l2 = Line::new(plot.origin_bl(1, 1), plot.origin_br(1, 1), '-');
            l1.draw(&plot);
            l2.draw(&plot);
            let l3 = Line::new(PVec2::new(2, 2), plot.origin_br(4, 2), '#');
            let l4 = Line::new(plot.origin_bl(2, 2), plot.origin_br(2, 10), '#');
            let l5 = Line::new(plot.origin_bl(6, 20), plot.origin_bl(12, 4), '*');
            l3.draw(&plot);
            l4.draw(&plot);
            l5.draw(&plot);
            plot.finish();
            Ok(())
        }
    }
}
