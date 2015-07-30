mod symbols;
mod segments;

use std::env;

use segments::connection::connect_last;
use segments::connection::connect_segments;

use segments::error::error_segment;
use segments::path::path_segments;

use segments::colors;

fn main() {
    let args: Vec<_> = env::args().collect();
    let error: bool = (args.len() > 1) && (args[1] != "0") ;

    let clean_colors : String = "\\[\x1b[0m\\]".to_string();
    let first_segment : String = error_segment(error);
    let second_segment : String = path_segments();

    println!("{clean}{error}{c0}{path}{c1}{clean} ",
             c0=connect_segments(colors::LAST_RESULT_BG,
                                 colors::PATH_BG),
             c1=connect_last(colors::PATH_BG),
             clean=clean_colors,
             error=first_segment,
             path=second_segment);
}
