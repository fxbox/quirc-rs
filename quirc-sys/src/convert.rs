use libc::{ c_int, uint8_t };

extern {
    pub fn yuyv_to_rgb32(src: *const uint8_t, src_pitch: c_int, w: c_int, h: c_int,
                         dst: *mut uint8_t, dst_pitch: c_int);
    pub fn yuyv_to_luma(src: *const uint8_t, src_pitch: c_int, w: c_int, h: c_int,
                         dst: *mut uint8_t, dst_pitch: c_int);
    pub fn rgb32_to_luma(src: *const uint8_t, src_pitch: c_int, w: c_int, h: c_int,
                         dst: *mut uint8_t, dst_pitch: c_int);
}
