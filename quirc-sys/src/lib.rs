extern crate libc;

use libc::{ c_int, uint8_t };
use std::ffi::CStr;
use std::fmt;
use std::io;
use std::mem;
use std::ptr;
use std::str;

pub mod convert;

#[macro_use]
pub mod utils;

pub mod ll {
    use libc::{ c_char, c_int, uint8_t };

    pub enum Quirc {}

    c_like_enum! {
        quirc_decode_error_t {
            QUIRC_SUCCESS = 0,
            QUIRC_ERROR_INVALID_GRID_SIZE = 1,
            QUIRC_ERROR_INVALID_VERSION = 2,
            QUIRC_ERROR_FORMAT_ECC = 3,
            QUIRC_ERROR_DATA_ECC = 4,
            QUIRC_ERROR_UNKNOWN_DATA_TYPE = 5,
            QUIRC_ERROR_DATA_OVERFLOW = 6,
            QUIRC_ERROR_DATA_UNDERFLOW = 7
        }
    } 

    // Limits on the maximum size of QR-codes and their content.
    pub const QUIRC_MAX_BITMAP: usize = 3917;
    pub const QUIRC_MAX_PAYLOAD: usize = 8896;

    // QR-code ECC types.
    pub const QUIRC_ECC_LEVEL_M: usize = 0;
    pub const QUIRC_ECC_LEVEL_L: usize = 1;
    pub const QUIRC_ECC_LEVEL_H: usize = 2;
    pub const QUIRC_ECC_LEVEL_Q: usize = 3;

    /* QR-code data types. */
    pub const QUIRC_DATA_TYPE_NUMERIC: usize = 1;
    pub const QUIRC_DATA_TYPE_ALPHA: usize = 2;
    pub const QUIRC_DATA_TYPE_BYTE: usize = 4;
    pub const QUIRC_DATA_TYPE_KANJI: usize = 8;

    #[repr(C)]
    pub struct QuircPoint {
        pub x: c_int,
        pub y: c_int,
    }

    #[repr(C)]
    pub struct QuircCode {
        pub corners: [QuircPoint; 4],
        pub size: c_int,
        pub cell_bitmap: [uint8_t; QUIRC_MAX_BITMAP],
    }

    #[repr(C)]
    pub struct QuircData {
        pub version: c_int,
        pub ecc_level: c_int,
        pub mask: c_int,
        pub data_type: c_int,
        pub payload: [uint8_t; QUIRC_MAX_PAYLOAD],
        pub payload_len: c_int,
    }

    extern {
      pub fn quirc_version() -> *const c_char;

      pub fn quirc_new() -> *mut Quirc;
      pub fn quirc_destroy(q: *mut Quirc);

      pub fn quirc_resize(q: *mut Quirc, w: c_int, h: c_int) -> c_int;
      pub fn quirc_begin(q: *mut Quirc, w: *mut c_int, h: *mut c_int) -> *mut uint8_t;
      pub fn quirc_end(q: *mut Quirc);
      pub fn quirc_count(q: *mut Quirc) -> c_int;
      pub fn quirc_extract(q: *mut Quirc, index: c_int, code: *mut QuircCode);
      pub fn quirc_decode(code: *const QuircCode, data: *mut QuircData) -> quirc_decode_error_t;
      pub fn quirc_strerror(err: quirc_decode_error_t) -> *const c_char;
    }
}

pub use ll::QuircCode as Code;
pub use ll::QuircData as Data;
pub use ll::quirc_decode_error_t as DecodeError;

pub struct Quirc {
    pub raw: *mut ll::Quirc,
}

impl Drop for Quirc {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe{ ll::quirc_destroy(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

impl Quirc {

    pub fn new() -> Self {
        Quirc {
            raw: unsafe { ll::quirc_new() },
        }
    }

    pub fn resize(&mut self, w: isize, h: isize) -> io::Result<()> {
        try_ue!(ll::quirc_resize(self.raw, w as c_int, h as c_int));
        Ok(())
    }

    pub fn begin(&mut self) -> *mut uint8_t {
        unsafe { ll::quirc_begin(self.raw, ptr::null_mut(), ptr::null_mut()) }
    }

    pub fn end(&mut self) {
        unsafe { ll::quirc_end(self.raw) };
    }

    pub fn count(&mut self) -> isize {
        unsafe { ll::quirc_count(self.raw) as isize }
    }

    pub fn extract(&mut self, index: isize) -> Code {
        unsafe {
            let mut code: Code = mem::zeroed();
            ll::quirc_extract(self.raw, index as c_int, &mut code as *mut Code);
            code
        }
    }
}

impl Code {
    pub fn decode(&self) -> Result<Data, DecodeError> {
        unsafe {
            let mut data: Data = mem::zeroed();
            let err = ll::quirc_decode(self as *const ll::QuircCode, &mut data as *mut Data);
            if err == DecodeError::QUIRC_SUCCESS {
                Ok(data)
            } else {
                Err(err)
            }
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{}", unsafe { CStr::from_ptr(ll::quirc_strerror(*self)).to_string_lossy() }))
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf: &[u8] = &self.payload[..(self.payload_len as usize)];
        f.pad(&format!("{}", str::from_utf8(buf).unwrap()))
    }
}

