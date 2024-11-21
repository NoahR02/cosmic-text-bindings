package cosmic_text
import "core:c"

when ODIN_OS == .Windows {
    foreign import cosmic_text {
        "../lib/cosmic_text_c.lib",
        "system:ntdll.lib",
		"system:userenv.lib",
		"system:ws2_32.lib",
    }
}

when ODIN_OS == .Linux {
    foreign import cosmic_text {
        "../lib/libcosmic_text_c.a",
    }
}

Shaping :: enum u32 {
    Basic,
    Advanced,
}

Stretch :: enum u32 {
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

Style :: enum u32 {
     Normal,
     Italic,
     Oblique,
}

Buffer :: struct {}
FontSystem :: struct {}
SwashCache :: struct {}

Color :: struct {
    self: u32
}

Family_Tag :: enum u32 {
    Name,
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
}

Family :: struct {
    tag: Family_Tag,
    u: struct #raw_union {
        name: struct {
            name: cstring
        }
    }
}

Weight :: struct {
    self: u16
}

CacheKeyFlags :: distinct u32
CacheKeyFlags_FAKE_ITALIC: u32 : 1

CacheMetrics :: struct {
    font_size_bits: u32,
    line_height_bits: u32
}

Attrs:: struct {
    color_opt: ^Color,
    family: Family,
    stretch: Stretch,
    style: Style,
    weight: Weight,
    metadata: c.size_t,
    cache_key_flags: CacheKeyFlags,
    metrics_opt: ^CacheMetrics
}

Metrics :: struct {
    font_size: f32,
    line_height: f32
}

ColorRGBA :: struct {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

foreign cosmic_text {
    attrs_constructor :: proc() -> Attrs ---
    buffer_constructor :: proc(font_system: ^^FontSystem, metrics: Metrics) -> ^Buffer ---
    buffer_destructor :: proc(buffer: ^Buffer) ---

    buffer_draw :: proc(
        buffer: ^Buffer,
        font_system: ^FontSystem,
        swash_cache: ^SwashCache,
        color: Color,
        void_ptr: rawptr,
        draw_fn: proc "cdecl" (rawptr, i32, i32, u32, u32, Color)
    ) ---

    buffer_set_size :: proc(buffer: ^Buffer, font_system: ^FontSystem, width: f32, height: f32) ---
    buffer_set_text :: proc(buffer: ^Buffer, font_system: ^FontSystem, text: cstring, attrs: Attrs, shaping: Shaping) ---
    buffer_shape_until_scroll :: proc(buffer: ^Buffer, font_system: ^FontSystem, prune: bool) ---
    
    color_r :: proc(self: Color) -> u8 ---
    color_g :: proc(self: Color) -> u8 ---
    color_b :: proc(self: Color) -> u8 ---
    color_a :: proc(self: Color) -> u8 ---
    color_as_rgba :: proc(self: Color) -> ColorRGBA ---
    color_rgba :: proc(r, g, b, a: u8) -> Color ---
    
    font_system_constructor :: proc() -> ^FontSystem ---
    font_system_destructor :: proc(font_system: ^FontSystem) ---
    metrics_constructor :: proc (font_size, line_height: f32) -> Metrics ---
    swash_cache_constructor :: proc() -> ^SwashCache ---
    swash_cache_destructor :: proc(swash_cache: ^SwashCache) ---
}