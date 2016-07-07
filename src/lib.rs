extern crate libc;

use std::fmt;
use std::io::{ self, Error, ErrorKind };
use std::num;

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(err: num::ParseIntError) -> ParseError {
        ParseError::Parse(err)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Io(ref err) => write!(f, "{}", err),
            ParseError::Parse(ref err) => write!(f, "{}", err),
        }
    }
}

pub fn parse_size(s: &str) -> Result<(u32, u32), ParseError> {
    let num: Vec<&str> = s.split('x').collect();
    if num.len() != 2 {
        return Err(ParseError::Io(Error::new(ErrorKind::InvalidInput, "Expected WxH")));
    }
    let w = try!(num[0].parse::<u32>());
    let h = try!(num[1].parse::<u32>());
    if w <= 0 || w >= 10000 || h <= 0 || h >= 10000 {
        return Err(ParseError::Io(Error::new(ErrorKind::InvalidInput, "Expected width and height to be 1-9999")));
    }
    return Ok((w, h));
}
