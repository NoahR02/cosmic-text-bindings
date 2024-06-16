use std::ffi::c_char;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color(pub u32);

#[no_mangle]
pub extern "C" fn color_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
}

#[no_mangle]
pub extern "C" fn color_r(color: Color) -> u8 {
    ((color.0 & 0x00_FF_00_00) >> 16) as u8
}
#[no_mangle]
pub extern "C" fn color_g(color: Color) -> u8 {
    ((color.0 & 0x00_00_FF_00) >> 8) as u8
}
#[no_mangle]
pub extern "C" fn color_b(color: Color) -> u8 {
    (color.0 & 0x00_00_00_FF) as u8
}
#[no_mangle]
pub extern "C" fn color_a(color: Color) -> u8 {
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
    Name(*const c_char),

    /// Serif fonts represent the formal text style for a script.
    Serif,

    /// Glyphs in sans-serif fonts, as the term is used in CSS, are generally low contrast
    /// and have stroke endings that are plain — without any flaring, cross stroke,
    /// or other ornamentation.
    SansSerif,

    /// Glyphs in cursive fonts generally use a more informal script style,
    /// and the result looks more like handwritten pen or brush writing than printed letterwork.
    Cursive,

    /// Fantasy fonts are primarily decorative or expressive fonts that
    /// contain decorative or expressive representations of characters.
    Fantasy,

    /// The sole criterion of a monospace font is that all glyphs have the same fixed width.
    Monospace,
}

#[repr(C)]
pub enum Stretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[repr(C)]
pub enum Style {
    /// A face that is neither italic not obliqued.
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

#[repr(C)]
pub struct Weight(pub u16);

#[repr(C)]
pub struct CacheMetrics {
    font_size_bits: u32,
    line_height_bits: u32,
}

bitflags::bitflags! {
    /// Flags that change rendering
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[repr(transparent)]
    pub struct CacheKeyFlags: u32 {
        /// Skew by 14 degrees to synthesize italic
        const FAKE_ITALIC = 1;
    }
}

#[repr(C)]
pub struct Attrs<'a>  {
    //TODO: should this be an option?
    pub color_opt: Option<&'a Color>,
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
pub extern "C" fn metrics_constructor(font_size: f32, line_height: f32) -> Metrics {
    Metrics {
        font_size,
        line_height
    }
}