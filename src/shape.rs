#[repr(C)]
pub enum Shaping {
    /// Basic shaping with no font fallback.
    ///
    /// This shaping strategy is very cheap, but it will not display complex
    /// scripts properly nor try to find missing glyphs in your system fonts.
    ///
    /// You should use this strategy when you have complete control of the text
    /// and the font you are displaying in your application.
    // #[cfg(feature = "swash")]
    // For now we will include this, but we should find a way to enable or disable this.
    Basic,
    /// Advanced text shaping and font fallback.
    ///
    /// You will need to enable this strategy if the text contains a complex
    /// script, the font used needs it, and/or multiple fonts in your system
    /// may be needed to display all of the glyphs.
    Advanced,
}

impl Shaping {
    pub(crate) fn convert_c_shaping(shaping: Shaping) -> cosmic_text::Shaping {
        match shaping {
            Shaping::Basic => cosmic_text::Shaping::Basic,
            Shaping::Advanced => cosmic_text::Shaping::Advanced
        }
    }
}
