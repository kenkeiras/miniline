use std::str;

/**
 * Draw an UTF-8 arrow.
 */
pub fn arrow() -> &'static str {
    str::from_utf8(b"\xee\x82\xb0").unwrap()
}

pub fn empty_arrow() -> &'static str {
    str::from_utf8(b"\xee\x82\xb1").unwrap()
}

pub fn cross() -> &'static str {
    str::from_utf8(b"\xe2\x9c\x97").unwrap()
}

pub fn tick() -> &'static str {
    str::from_utf8(b"\xe2\x9c\x93").unwrap()
}
