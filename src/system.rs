pub struct FontSystem {}

impl FontSystem {
    #[no_mangle]
    pub extern "C" fn font_system_constructor() -> *mut cosmic_text::FontSystem {
        let lib_font_system = cosmic_text::FontSystem::new();
        Box::into_raw(Box::new(lib_font_system))
    }
    #[no_mangle]
    pub extern "C" fn font_system_destructor(font_system: *mut cosmic_text::FontSystem) {
        unsafe { let _ = Box::from_raw(font_system); }
    }
}