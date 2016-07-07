use libc::{ c_int, c_void, int32_t };
use std::io;
use std::ptr;

extern {
    pub fn mjpeg_decoder_alloc() -> *mut c_void;
    pub fn mjpeg_decoder_free(mj: *mut c_void);

    pub fn mjpeg_init(mj: *mut c_void);
    pub fn mjpeg_free(mj: *mut c_void); // frees internal memory

    pub fn mjpeg_decode_rgb32(ml: *mut c_void,
                              data: *const c_void, datalen: c_int, out: *mut c_void,
                              pitch: c_int, max_w: c_int, max_h: c_int) -> c_int;

    pub fn mjpeg_decode_gray(mj: *mut c_void,
                             data: *const c_void, datalen: c_int, out: *mut c_void,
                             pitch: c_int, max_w: c_int, max_h: c_int) -> c_int;

}

pub struct MjpegDecoder {
    decoder: *mut c_void,
}

impl MjpegDecoder {

    pub fn new() -> Self {
        let decoder = unsafe { mjpeg_decoder_alloc() };
        unsafe { mjpeg_init(decoder) };
        MjpegDecoder {
            decoder: decoder,
        }
    }

    pub fn free(&mut self) {
        if !self.decoder.is_null() {
            unsafe { mjpeg_free(self.decoder); };
            unsafe { mjpeg_decoder_free(self.decoder) };
            self.decoder = ptr::null_mut();
        }   
    }

    pub fn decode_rgb32(&mut self, data: *const c_void, datalen: usize, out: *mut c_void,
                        pitch: int32_t, max_w: int32_t, max_h: int32_t) -> io::Result<()> {
        if unsafe { mjpeg_decode_rgb32(self.decoder, data, datalen as c_int, out, pitch, max_w, max_h) } < 0 {
            Err(io::Error::new(io::ErrorKind::InvalidData, "MJPEG: frame too big"))
        } else {
            Ok(())
        }
    }

    pub fn decode_gray(&mut self, data: *const c_void, datalen: usize, out: *mut c_void,
                        pitch: int32_t, max_w: int32_t, max_h: int32_t) -> io::Result<()> {
        if unsafe { mjpeg_decode_gray(self.decoder, data, datalen as c_int, out, pitch, max_w, max_h) } < 0 {
            Err(io::Error::new(io::ErrorKind::InvalidData, "MJPEG: frame too big"))
        } else {
            Ok(())
        }
    }

}

impl Drop for MjpegDecoder {

    fn drop(&mut self) {
        self.free();
    }
}
