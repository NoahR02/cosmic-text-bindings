#include <stdio.h>
#include "cosmic-text.h"
#include "raylib.h"

void draw_fn(int32_t x, int32_t y, uint32_t w, uint32_t h, CosmicTextColor color) {
    Color rayib_color;
    rayib_color.r = color_r(color);
    rayib_color.g = color_g(color);
    rayib_color.b = color_b(color);
    rayib_color.a = color_a(color);

    DrawRectangle(x, y, w, h, rayib_color);
    printf("x: {%d}, y: {%d}", x, y);
}

int main() {

    Metrics a = metrics_new(20.0f, 40.0f);

    printf("{%f}", a.font_size);
    printf("{%f}", a.line_height);

    FontSystem* font_system = calloc(1, sizeof(FontSystem*));
    font_system_new(&font_system);

    SwashCache* swash_cache = calloc(1, sizeof(SwashCache*));
    swash_cache_new(&swash_cache);

    Buffer* buffer = calloc(1, sizeof(Buffer*));
    buffer_new(&buffer, &font_system, a);
    buffer_set_size(buffer, font_system, 1000.0f, 500.0f);
    Attrs attrs = attrs_new();
    buffer_set_text(buffer, font_system, "Hello, C! \n ❄", attrs, ShapingAdvanced);
    buffer_shape_until_scroll(buffer, font_system, true);

    CosmicTextColor text_color = color_rgba(0, 0, 0, 255);

    const int screenWidth = 900;
    const int screenHeight = 450;

    InitWindow(screenWidth, screenHeight, "cosmic-text-raylib-example");
    SetTargetFPS(60);

    // Main game loop
    while (!WindowShouldClose())
    {
        BeginDrawing();

        ClearBackground(WHITE);
        buffer_draw(buffer, font_system, swash_cache, text_color, draw_fn);
        EndDrawing();
    }

    CloseWindow();
    return 0;
}