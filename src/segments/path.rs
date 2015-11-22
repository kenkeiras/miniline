use std::env;
use std::path::Path;
use std::path::PathBuf;

use std::borrow::Borrow;
use symbols::empty_arrow;

use segments::colors;

fn relative_path_segments(base : &PathBuf, part : &PathBuf) -> String {
    match (base.to_str(), part.to_str()) {
        (Some(ref base_str), Some(ref part_str)) => {
            let mut path_from_home : String = "~/".to_string();
            path_from_home.push_str(&base_str[part_str.len()..].to_owned());

            path_segments_from_root(&Path::new(&path_from_home))
        },
        (_, _) => {
            path_segments_from_root(&base)
        }
    }
}


fn path_segments_from_home(cwd : &PathBuf, home : &PathBuf) -> String {

    let segment : String = relative_path_segments(cwd, home);

    segment
}


fn path_segments_from_root(cwd : &Path) -> String {
    let mut segment : String = "".to_string();
    let mut first : bool = true;

    for component in cwd.components() {
        let part : &str = match component.as_ref().to_str() {
            None  => "???",
            Some(ref dir) => {
                dir
            }
        };

        if first {
            first = false;
        }
        else { // Add followup from the last segment
            segment.push_str(
                format!(" \\[\x1b[0;3{arrow_color};4{bg}m\\]{arrow}",
                        arrow_color=colors::PATH_ARROWS,
                        bg=colors::PATH_BG,
                        arrow=empty_arrow())
                    .borrow());
        }

        segment.push_str(
            format!("\\[\x1b[0;38;5;1{fg};4{bg}m\\] {part}",
                    fg=colors::PATH_FG,
                    bg=colors::PATH_BG,
                    part=part)
                .borrow());
    }

    segment
}


/**
 * Build a segment displaying the current path.
 */
pub fn path_segments() -> String {
    let cwd = env::current_dir().unwrap();

    let segments : String = match env::home_dir() {
        None => path_segments_from_root(cwd.as_path()),
        Some(ref h) => {
            if cwd.starts_with(h) {
                path_segments_from_home(&cwd, h)
            }
            else {
                path_segments_from_root(cwd.as_path())
            }
        }
    };

    segments
}
