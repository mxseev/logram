use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ANSI_CODE_REGEX: Regex = Regex::new("\x1b\\[[^@-~]*[@-~]").unwrap();
}

pub fn crop_ansi_codes(input: &str) -> String {
    ANSI_CODE_REGEX.replace_all(input, "").to_string()
}
