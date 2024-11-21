#include <cosmic-text.hpp>
#include "raylib.h"

constexpr int SCREEN_WIDTH = 900;
constexpr int SCREEN_HEIGHT = 450;
constexpr int WINDOW_PADDING = 20;

void draw_fn(void* user_data, int32_t x, int32_t y, uint32_t w, uint32_t h, cosmic_text::Color color) {
    const auto color_rgba = cosmic_text::color_as_rgba(color);
    const Color rayib_color = {color_rgba.r, color_rgba.g, color_rgba.b, color_rgba.a};

    DrawRectangle(x + WINDOW_PADDING, y + WINDOW_PADDING, static_cast<int>(w), static_cast<int>(h), rayib_color);
}

int main() {

    cosmic_text::Metrics a = cosmic_text::metrics_constructor(20.0f, 40.0f);

    // Opaque pointers.
    auto font_system = cosmic_text::font_system_constructor();
    auto swash_cache = cosmic_text::swash_cache_constructor();
    auto buffer = cosmic_text::buffer_constructor(&font_system, a);

    cosmic_text::buffer_set_size(buffer, font_system, SCREEN_WIDTH - WINDOW_PADDING, SCREEN_HEIGHT - WINDOW_PADDING);
    cosmic_text::Attrs attrs = cosmic_text::attrs_constructor();
    cosmic_text::buffer_set_text(buffer, font_system, "Hello, C++!\nおはよう (ja) (ohayō) (morning), こんにちは (ja) (konnichi wa) (daytime), こんばんは (ja) (konban wa) (evening)", attrs, cosmic_text::Shaping::Advanced);
    cosmic_text::buffer_shape_until_scroll(buffer, font_system, true);

    const cosmic_text::Color text_color = cosmic_text::color_rgba(0, 0, 0, 255);

    SetTraceLogLevel(TraceLogLevel::LOG_WARNING);
    InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "cosmic-text-raylib-example");
    SetTargetFPS(60);

    while (!WindowShouldClose())
    {
        BeginDrawing();

        ClearBackground(WHITE);
        cosmic_text::buffer_draw(buffer, font_system, swash_cache, text_color, nullptr, draw_fn);
        EndDrawing();
    }

    // Make sure to release your memory!
    cosmic_text::buffer_destructor(buffer);
    cosmic_text::swash_cache_destructor(swash_cache);
    cosmic_text::font_system_destructor(font_system);

    CloseWindow();
    return 0;
}