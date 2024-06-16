use std::ffi::{c_char, CStr};
use crate::attrs::{Attrs, CacheKeyFlags, Color, Family, Metrics, Stretch, Style, Weight};
use crate::shape::Shaping;
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

fn convert_c_shaping(shaping: Shaping) -> cosmic_text::Shaping {
    match shaping {
        Shaping::Basic => cosmic_text::Shaping::Basic,
        Shaping::Advanced => cosmic_text::Shaping::Advanced
    }
}

fn convert_c_attrs<'a>(attrs: Attrs) -> cosmic_text::Attrs<'a> {

    let color: Option<cosmic_text::Color> = {
        let opt = if attrs.color_opt.is_none() {
            None
        } else {
            Some(cosmic_text::Color(attrs.color_opt.unwrap().0))
        };
        opt
    };

    let style: cosmic_text::Style = match attrs.style {
        Style::Normal => cosmic_text::Style::Normal,
        Style::Italic => cosmic_text::Style::Italic,
        Style::Oblique => cosmic_text::Style::Oblique
    };

    let stretch = match attrs.stretch {
        Stretch::UltraCondensed => cosmic_text::Stretch::UltraCondensed,
        Stretch::ExtraCondensed => cosmic_text::Stretch::ExtraCondensed,
        Stretch::Condensed      => cosmic_text::Stretch::Condensed,
        Stretch::SemiCondensed  => cosmic_text::Stretch::SemiCondensed,
        Stretch::Normal         => cosmic_text::Stretch::Normal,
        Stretch::SemiExpanded   => cosmic_text::Stretch::SemiExpanded,
        Stretch::Expanded       => cosmic_text::Stretch::Expanded,
        Stretch::ExtraExpanded  => cosmic_text::Stretch::ExtraExpanded,
        Stretch::UltraExpanded  => cosmic_text::Stretch::UltraExpanded
    };
    
    let family: cosmic_text::Family = match attrs.family {
        Family::Name(name) => unsafe {
            cosmic_text::Family::Name(CStr::from_ptr(name).to_str().unwrap())
        },
        Family::Serif => cosmic_text::Family::Serif,
        Family::SansSerif => cosmic_text::Family::SansSerif,
        Family::Cursive => cosmic_text::Family::Cursive,
        Family::Fantasy => cosmic_text::Family::Fantasy,
        Family::Monospace => cosmic_text::Family::Monospace,
    };

    cosmic_text::Attrs {
        color_opt: color,
        family,
        stretch,
        style,
        weight: cosmic_text::Weight(attrs.weight.0),
        metadata: attrs.metadata,
        cache_key_flags: cosmic_text::CacheKeyFlags::from_bits_retain(attrs.cache_key_flags.bits()),
        metrics_opt: None,
    }
}

#[no_mangle]
pub extern "C" fn attrs_constructor() -> Attrs<'static>  {
    Attrs {
        color_opt: None,
        family: Family::SansSerif,
        stretch: Stretch::Normal,
        style: Style::Normal,
        weight: Weight(400),
        metadata: 0,
        cache_key_flags: CacheKeyFlags::empty(),
        metrics_opt: None,
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
        buffer.set_text(font_system, c_str, convert_c_attrs(attrs), convert_c_shaping(shaping));
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
    pub extern "C" fn buffer_draw(buffer: &mut cosmic_text::Buffer, font_system: &mut cosmic_text::FontSystem, swash_cache: &mut cosmic_text::SwashCache, color: Color, draw_fn: extern fn(i32, i32, u32, u32, Color)) {
        let lib_color = cosmic_text::Color(color.0);
        buffer.draw(font_system, swash_cache, lib_color, |x, y, w, h, color| {
            let c_color = Color(color.0);
            draw_fn(x, y, w, h, c_color);
        });
    }
}