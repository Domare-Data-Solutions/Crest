
use std::ffi::OsString;
use std::fs::read_to_string;

use peacock_crest::{css, ast};

fn main() {
    let dir = "static/css/style.css";
    let content_raw = read_to_string(dir).unwrap();
    let content_result = css::CssParser::new().parse(&content_raw);
    if content_result.is_err() {
        let err = content_result.err().unwrap();
        println!("{err}");
        return;
    }
    let content = content_result.unwrap();
    for rule in content.iter() {
        let mut selectors = String::new();
        for selector in rule.selectors.iter() {
            // selectors += format!(" {}", selector.to_string());
        }
    }
}
