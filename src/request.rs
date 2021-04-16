use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REQUEST_REGEX: Regex = Regex::new(r"^([A-Z]+) ([^ ]+) HTTP/(\d+.\d+)").unwrap();
}

#[derive(Debug, Clone)]
pub struct Request<'a> {
    pub method: &'a str,
    pub url: &'a str,
    pub version: &'a str,
}

pub fn parse_request_line<'a>(line: &'a str) -> Result<Request<'a>, ()> {
    let captures = REQUEST_REGEX.captures(line).ok_or(())?;

    Ok(Request {
        method: captures.get(1).unwrap().as_str(),
        url: captures.get(2).unwrap().as_str(),
        version: captures.get(3).unwrap().as_str(),
    })
}
