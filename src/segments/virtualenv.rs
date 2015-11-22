use std::env;
use segments::colors;

/**
 * Add the chosen background and foreground colors to the strings.
 */
fn wrap(str: &str) -> String {
    format!("\\[\x1b[9{fg};4{bg}m\\] {str}",
            fg=colors::VIRTUALENV_FG,
            bg=colors::VIRTUALENV_BG,
            str=str)
}

/**
 * Show just the last part of the virtualenv path.
 */
fn cut(x: String) -> Option<String> {
    let s : Vec<&str> = x.rsplit("/").collect();

    // If the virtualenv is empty
    if s.len() == 0 {
        None
    }
    // If the virtualenv is not ended by a /
    else if s[0].trim().len() > 0 {
        Some(wrap(s[0].trim()))
    }
    // If there's a second directory to be used
    else if s.len() > 1 {
        Some(wrap(s[1].trim()))
    }
    else {
        None
    }
}

/**
 * Build a segment displaying the current path.
 */
pub fn virtualenv_segment() -> Option<String> {
    for (k, v) in env::vars() {
        if k == "VIRTUAL_ENV" {
            return cut(v)
        }
    }

    None
}
