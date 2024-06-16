use std::ffi::{c_char, CStr};
use crate::attrs::{Attrs, CacheKeyFlags, CosmicTextColor, Family, Metrics, Stretch, Style, Weight};
use crate::shape::Shaping;
pub struct Buffer {}
pub struct SwashCache {}

#[no_mangle]
pub extern "C" fn swash_cache_new(swash_cache: *mut *mut cosmic_text::SwashCache) -> u32 {

    let lib_swash_cache = cosmic_text::SwashCache::new();
    
    if swash_cache.is_null() {
        // Null Pointer
        return 0;
    }

    unsafe {
        *swash_cache = std::ptr::null_mut();
    }

    unsafe {
        *swash_cache = Box::into_raw(Box::new(lib_swash_cache));
        // Valid Pointer
        1
    }
}

#[no_mangle]
pub extern "C" fn buffer_new(buffer: *mut *mut cosmic_text::Buffer, font_system: *mut *mut cosmic_text::FontSystem, metrics: Metrics) -> u32 {

    let inner_font_system: &'static mut cosmic_text::FontSystem = unsafe {
        let inner_ptr: *mut cosmic_text::FontSystem = *font_system;
        &mut *inner_ptr
    };

    let lib_metrics = cosmic_text::Metrics::new(metrics.font_size, metrics.line_height);
    let lib_buffer = cosmic_text::Buffer::new(inner_font_system, lib_metrics);

    if buffer.is_null() {
        // Null Pointer
        return 0;
    }

    unsafe {
        *buffer = std::ptr::null_mut();
    }

    unsafe {
        *buffer = Box::into_raw(Box::new(lib_buffer));
        // Valid Pointer
        1
    }
}

fn convert_c_shaping(shaping: Shaping) -> cosmic_text::Shaping {
    match shaping {
        Shaping::ShapingBasic => cosmic_text::Shaping::Basic,
        Shaping::ShapingAdvanced => cosmic_text::Shaping::Advanced
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
        Style::StyleNormal => cosmic_text::Style::Normal,
        Style::StyleItalic => cosmic_text::Style::Italic,
        Style::StyleOblique => cosmic_text::Style::Oblique
    };

    let stretch = match attrs.stretch {
        Stretch::StretchUltraCondensed => cosmic_text::Stretch::UltraCondensed,
        Stretch::StretchExtraCondensed => cosmic_text::Stretch::ExtraCondensed,
        Stretch::StretchCondensed      => cosmic_text::Stretch::Condensed,
        Stretch::StretchSemiCondensed  => cosmic_text::Stretch::SemiCondensed,
        Stretch::StretchNormal         => cosmic_text::Stretch::Normal,
        Stretch::StretchSemiExpanded   => cosmic_text::Stretch::SemiExpanded,
        Stretch::StretchExpanded       => cosmic_text::Stretch::Expanded,
        Stretch::StretchExtraExpanded  => cosmic_text::Stretch::ExtraExpanded,
        Stretch::StretchUltraExpanded  => cosmic_text::Stretch::UltraExpanded
    };
    
    let family: cosmic_text::Family = match attrs.family {
        Family::FamilyName(name) => unsafe {
            cosmic_text::Family::Name(CStr::from_ptr(name).to_str().unwrap())
        },
        Family::FamilySerif => cosmic_text::Family::Serif,
        Family::FamilySansSerif => cosmic_text::Family::SansSerif,
        Family::FamilyCursive => cosmic_text::Family::Cursive,
        Family::FamilyFantasy => cosmic_text::Family::Fantasy,
        Family::FamilyMonospace => cosmic_text::Family::Monospace,
    };

    cosmic_text::Attrs {
        color_opt: color,
        family,
        stretch,
        style,
        weight: cosmic_text::Weight(attrs.weight.0),
        metadata: attrs.metadata,
        cache_key_flags: cosmic_text::CacheKeyFlags::from_bits_retain(attrs.cache_key_flags.0),
        metrics_opt: None,
    }
}

#[no_mangle]
pub extern "C" fn attrs_new() -> Attrs<'static>  {
    Attrs {
        color_opt: None,
        family: Family::FamilySansSerif,
        stretch: Stretch::StretchNormal,
        style: Style::StyleNormal,
        weight: Weight(400),
        metadata: 0,
        cache_key_flags: CacheKeyFlags(0),
        metrics_opt: None,
    }
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
pub extern "C" fn buffer_draw(buffer: &mut cosmic_text::Buffer, font_system: &mut cosmic_text::FontSystem, swash_cache: &mut cosmic_text::SwashCache, color: CosmicTextColor, draw_fn: fn(i32, i32, u32, u32, CosmicTextColor)) {
    let lib_color = cosmic_text::Color(color.0);
    buffer.draw(font_system, swash_cache, lib_color, |x, y, w, h, color| {
        let c_color = CosmicTextColor(color.0);
        draw_fn(x, y, w, h, c_color);
    });
}