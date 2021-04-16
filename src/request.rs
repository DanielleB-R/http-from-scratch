use http::{request::Builder, Request};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REQUEST_REGEX: Regex = Regex::new(r"^([A-Z]+) ([^ ]+) HTTP/(\d+.\d+)").unwrap();
}

pub fn parse_request_line(line: &str) -> Result<Builder, ()> {
    let captures = REQUEST_REGEX.captures(line).ok_or(())?;

    Ok(Request::builder()
        .method(captures.get(1).unwrap().as_str())
        .uri(captures.get(2).unwrap().as_str()))
}
