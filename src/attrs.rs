use std::ffi::c_char;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmicTextColor(pub u32);

#[no_mangle]
pub extern "C" fn color_rgba(r: u8, g: u8, b: u8, a: u8) -> CosmicTextColor {
    CosmicTextColor(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
}

#[no_mangle]
pub extern "C" fn color_r(color: CosmicTextColor) -> u8 {
    ((color.0 & 0x00_FF_00_00) >> 16) as u8
}
#[no_mangle]
pub extern "C" fn color_g(color: CosmicTextColor) -> u8 {
    ((color.0 & 0x00_00_FF_00) >> 8) as u8
}
#[no_mangle]
pub extern "C" fn color_b(color: CosmicTextColor) -> u8 {
    (color.0 & 0x00_00_00_FF) as u8
}
#[no_mangle]
pub extern "C" fn color_a(color: CosmicTextColor) -> u8 {
    ((color.0 & 0xFF_00_00_00) >> 24) as u8
}

#[repr(C)]
pub enum Family {
    /// The name of a font family of choice.
    ///
    /// This must be a *Typographic Family* (ID 16) or a *Family Name* (ID 1) in terms of TrueType.
    /// Meaning you have to pass a family without any additional suffixes like _Bold_, _Italic_,
    /// _Regular_, etc.
    ///
    /// Localized names are allowed.
    FamilyName(*const c_char),

    /// Serif fonts represent the formal text style for a script.
    FamilySerif,

    /// Glyphs in sans-serif fonts, as the term is used in CSS, are generally low contrast
    /// and have stroke endings that are plain — without any flaring, cross stroke,
    /// or other ornamentation.
    FamilySansSerif,

    /// Glyphs in cursive fonts generally use a more informal script style,
    /// and the result looks more like handwritten pen or brush writing than printed letterwork.
    FamilyCursive,

    /// Fantasy fonts are primarily decorative or expressive fonts that
    /// contain decorative or expressive representations of characters.
    FamilyFantasy,

    /// The sole criterion of a monospace font is that all glyphs have the same fixed width.
    FamilyMonospace,
}

#[repr(C)]
pub enum Stretch {
    StretchUltraCondensed,
    StretchExtraCondensed,
    StretchCondensed,
    StretchSemiCondensed,
    StretchNormal,
    StretchSemiExpanded,
    StretchExpanded,
    StretchExtraExpanded,
    StretchUltraExpanded,
}

#[repr(C)]
pub enum Style {
    /// A face that is neither italic not obliqued.
    StyleNormal,
    /// A form that is generally cursive in nature.
    StyleItalic,
    /// A typically-sloped version of the regular face.
    StyleOblique,
}

#[repr(C)]
pub struct Weight(pub u16);

#[repr(C)]
pub struct CacheMetrics {
    font_size_bits: u32,
    line_height_bits: u32,
}

#[repr(C)]
pub struct CacheKeyFlags(pub u32);

/// Skew by 14 degrees to synthesize italic
pub const CACHE_KEY_FLAG_FAKE_ITALIC: u32 = 1;

#[repr(C)]
pub struct Attrs<'a>  {
    //TODO: should this be an option?
    pub color_opt: Option<&'a CosmicTextColor>,
    pub family: Family,
    pub stretch: Stretch,
    pub style: Style,
    pub weight: Weight,
    pub metadata: usize,
    pub cache_key_flags: CacheKeyFlags,
    pub metrics_opt: Option<&'a CacheMetrics>,
}

#[repr(C)]
pub struct Metrics {
    /// Font size in pixels
    pub font_size: f32,
    /// Line height in pixels
    pub line_height: f32,
}

#[no_mangle]
pub extern "C" fn metrics_new(font_size: f32, line_height: f32) -> Metrics {
    Metrics {
        font_size,
        line_height
    }
}