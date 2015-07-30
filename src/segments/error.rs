use symbols::cross;
use symbols::tick;
use segments::colors;

/**
 * Build a segment specifying if the last command was succesful.
 */
pub fn error_segment(error: bool) -> String {
    let mut segment : String = "".to_string();
    if error {
        segment.push_str("\\[\x1b[91;4");
        segment.push_str(colors::LAST_RESULT_BG);
        segment.push_str("m\\] ");

        segment.push_str(cross());
    }
    else {
        segment.push_str("\\[\x1b[92;4");
        segment.push_str(colors::LAST_RESULT_BG);
        segment.push_str("m\\] ");

        segment.push_str(tick());
    }

    segment.push_str(" ");
    segment
}
