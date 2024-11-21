use std::ffi::{c_char, CStr};

use crate::attrs::{Attrs, Color, Metrics};
use crate::shape::{Shaping};

pub struct SwashCache {}
pub struct Buffer {}

impl SwashCache {
    #[no_mangle]
    pub extern "C" fn swash_cache_constructor() -> *mut cosmic_text::SwashCache {
        let lib_swash_cache = cosmic_text::SwashCache::new();
        Box::into_raw(Box::new(lib_swash_cache))
    }
    #[no_mangle]
    pub extern "C" fn swash_cache_destructor(swash_cache: &mut cosmic_text::SwashCache) {
        unsafe { let _ = Box::from_raw(swash_cache); }
    }
}

impl Buffer {
    #[no_mangle]
    pub extern "C" fn buffer_constructor(font_system: *mut *mut cosmic_text::FontSystem, metrics: Metrics) -> *mut cosmic_text::Buffer {
        let inner_font_system: &'static mut cosmic_text::FontSystem = unsafe {
            let inner_ptr: *mut cosmic_text::FontSystem = *font_system;
            &mut *inner_ptr
        };

        let lib_metrics = cosmic_text::Metrics::new(metrics.font_size, metrics.line_height);
        let lib_buffer = cosmic_text::Buffer::new(inner_font_system, lib_metrics);

        Box::into_raw(Box::new(lib_buffer))
    }

    #[no_mangle]
    pub extern "C" fn buffer_destructor(buffer: *mut cosmic_text::Buffer) {
        unsafe { let _ = Box::from_raw(buffer); }
    }

    #[no_mangle]
    pub extern "C" fn buffer_set_text(buffer: &mut cosmic_text::Buffer, font_system: &mut cosmic_text::FontSystem, text: *const c_char, attrs: Attrs, shaping: Shaping) {
        let c_str = unsafe { CStr::from_ptr(text).to_str().unwrap() };
        buffer.set_text(font_system, c_str, Attrs::convert_c_attrs(attrs), Shaping::convert_c_shaping(shaping));
    }

    #[no_mangle]
    pub extern "C" fn buffer_set_size(buffer: &mut cosmic_text::Buffer, font_system: &mut cosmic_text::FontSystem, width: f32, height: f32) {
        // Set a size for the text buffer, in pixels
        buffer.set_size(font_system, Some(width), Some(height));
    }
    
    #[no_mangle]
    pub extern "C" fn buffer_shape_until_scroll(buffer: &mut cosmic_text::Buffer, font_system: &mut cosmic_text::FontSystem, prune: bool) {
        buffer.shape_until_scroll(font_system, prune);
    }

    #[no_mangle]
    pub extern "C" fn buffer_draw(buffer: &mut cosmic_text::Buffer,
                                  font_system: &mut cosmic_text::FontSystem,
                                  swash_cache: &mut cosmic_text::SwashCache,
                                  color: Color,
                                  void_ptr: *mut core::ffi::c_void,
                                  draw_fn: extern fn(*mut core::ffi::c_void, i32, i32, u32, u32, Color)) {
        let lib_color = cosmic_text::Color(color.0);
        buffer.draw(font_system, swash_cache, lib_color, |x, y, w, h, color| {
            let c_color = Color(color.0);
            draw_fn(void_ptr, x, y, w, h, c_color);
        });
    }
}