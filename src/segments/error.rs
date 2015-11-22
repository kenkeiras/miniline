use symbols::cross;
use symbols::tick;
use segments::colors;

/**
 * Build a segment specifying if the last command was succesful.
 */
pub fn error_segment(error: bool) -> String {
    if error {
        format!("\\[\x1b[91;4{bg}m\\] {cross}",
                bg=colors::LAST_RESULT_BG,
                cross=cross())
    }
    else {
        format!("\\[\x1b[92;4{bg}m\\] {tick}",
                bg=colors::LAST_RESULT_BG,
                tick=tick())
    }
}
