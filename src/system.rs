pub struct FontSystem {}

#[no_mangle]
pub extern "C" fn font_system_new(font_system: *mut *mut cosmic_text::FontSystem) -> u32 {

    let lib_font_system = cosmic_text::FontSystem::new();

    if font_system.is_null() {
        // Null Pointer
        return 0;
    }

    unsafe {
        *font_system = std::ptr::null_mut();
    }

    unsafe {
        *font_system = Box::into_raw(Box::new(lib_font_system));
        // Valid Pointer
        1
    }
}