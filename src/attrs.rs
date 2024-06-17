use std::ffi::{c_char, CStr};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color(pub u32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    #[inline]
    #[no_mangle]
    pub extern "C" fn color_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    #[inline]
    #[no_mangle]
    pub extern "C" fn color_r(self) -> u8 {
        ((self.0 & 0x00_FF_00_00) >> 16) as u8
    }
    #[inline]
    #[no_mangle]
    pub extern "C" fn color_g(self) -> u8 {
        ((self.0 & 0x00_00_FF_00) >> 8) as u8
    }

    #[inline]
    #[no_mangle]
    pub extern "C" fn color_b(self) -> u8 {
        (self.0 & 0x00_00_00_FF) as u8
    }

    #[inline]
    #[no_mangle]
    pub extern "C" fn color_a(self) -> u8 {
        ((self.0 & 0xFF_00_00_00) >> 24) as u8
    }

    #[inline]
    #[no_mangle]
    pub extern "C" fn color_as_rgba(self) -> ColorRGBA {
        ColorRGBA {
            r: self.color_r(),
            g: self.color_g(),
            b: self.color_b(),
            a: self.color_a()
        }
    }
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

// cbindgen can't generate the binding(s) for attrs_* if it is inside a complex impl.
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

impl<'a> Attrs<'a> {

    pub(crate) fn convert_c_attrs(attrs: Attrs) -> cosmic_text::Attrs<'a> {

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
}

#[repr(C)]
pub struct Metrics {
    /// Font size in pixels
    pub font_size: f32,
    /// Line height in pixels
    pub line_height: f32,
}

impl Metrics {
    #[no_mangle]
    pub extern "C" fn metrics_constructor(font_size: f32, line_height: f32) -> Metrics {
        Metrics {
            font_size,
            line_height
        }
    }
}