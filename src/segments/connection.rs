use symbols::arrow;


pub fn connect_segments(from_color : &str,
                        to_color : &str ) -> String {

    let mut connection : String = "\\[\x1b[0;3".to_string();
    connection.push_str(from_color);
    connection.push_str(";4");
    connection.push_str(to_color);
    connection.push_str("m\\]");

    connection.push_str(arrow());

    connection.push_str("\\[\x1b[0m\\]");

    connection
}

pub fn connect_last(from_color : &str) -> String {

    let mut connection : String = "\\[\x1b[0;3".to_string();
    connection.push_str(from_color);
    connection.push_str("m\\]");

    connection.push_str(arrow());

    connection.push_str("\\[\x1b[0m\\]");

    connection
}
