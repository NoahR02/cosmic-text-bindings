#ifndef cosmic_text_h
#define cosmic_text_h

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


/**
 * Skew by 14 degrees to synthesize italic
 */
#define CACHE_KEY_FLAG_FAKE_ITALIC 1

typedef enum Shaping {
  /**
   * Basic shaping with no font fallback.
   *
   * This shaping strategy is very cheap, but it will not display complex
   * scripts properly nor try to find missing glyphs in your system fonts.
   *
   * You should use this strategy when you have complete control of the text
   * and the font you are displaying in your application.
   */
  ShapingBasic,
  /**
   * Advanced text shaping and font fallback.
   *
   * You will need to enable this strategy if the text contains a complex
   * script, the font used needs it, and/or multiple fonts in your system
   * may be needed to display all of the glyphs.
   */
  ShapingAdvanced,
} Shaping;

typedef enum Stretch {
  StretchUltraCondensed,
  StretchExtraCondensed,
  StretchCondensed,
  StretchSemiCondensed,
  StretchNormal,
  StretchSemiExpanded,
  StretchExpanded,
  StretchExtraExpanded,
  StretchUltraExpanded,
} Stretch;

typedef enum Style {
  /**
   * A face that is neither italic not obliqued.
   */
  StyleNormal,
  /**
   * A form that is generally cursive in nature.
   */
  StyleItalic,
  /**
   * A typically-sloped version of the regular face.
   */
  StyleOblique,
} Style;

typedef struct Buffer Buffer;

typedef struct FontSystem FontSystem;

typedef struct SwashCache SwashCache;

typedef struct CosmicTextColor {
  uint32_t _0;
} CosmicTextColor;

typedef enum Family_Tag {
  /**
   * The name of a font family of choice.
   *
   * This must be a *Typographic Family* (ID 16) or a *Family Name* (ID 1) in terms of TrueType.
   * Meaning you have to pass a family without any additional suffixes like _Bold_, _Italic_,
   * _Regular_, etc.
   *
   * Localized names are allowed.
   */
  FamilyName,
  /**
   * Serif fonts represent the formal text style for a script.
   */
  FamilySerif,
  /**
   * Glyphs in sans-serif fonts, as the term is used in CSS, are generally low contrast
   * and have stroke endings that are plain — without any flaring, cross stroke,
   * or other ornamentation.
   */
  FamilySansSerif,
  /**
   * Glyphs in cursive fonts generally use a more informal script style,
   * and the result looks more like handwritten pen or brush writing than printed letterwork.
   */
  FamilyCursive,
  /**
   * Fantasy fonts are primarily decorative or expressive fonts that
   * contain decorative or expressive representations of characters.
   */
  FamilyFantasy,
  /**
   * The sole criterion of a monospace font is that all glyphs have the same fixed width.
   */
  FamilyMonospace,
} Family_Tag;

typedef struct Family {
  Family_Tag tag;
  union {
    struct {
      const char *family_name;
    };
  };
} Family;

typedef struct Weight {
  uint16_t _0;
} Weight;

typedef struct CacheKeyFlags {
  uint32_t _0;
} CacheKeyFlags;

typedef struct CacheMetrics {
  uint32_t font_size_bits;
  uint32_t line_height_bits;
} CacheMetrics;

typedef struct Attrs {
  const struct CosmicTextColor *color_opt;
  struct Family family;
  enum Stretch stretch;
  enum Style style;
  struct Weight weight;
  size_t metadata;
  struct CacheKeyFlags cache_key_flags;
  const struct CacheMetrics *metrics_opt;
} Attrs;

typedef struct Metrics {
  /**
   * Font size in pixels
   */
  float font_size;
  /**
   * Line height in pixels
   */
  float line_height;
} Metrics;

struct Attrs attrs_new(void);

void buffer_draw(struct Buffer *buffer,
                 struct FontSystem *font_system,
                 struct SwashCache *swash_cache,
                 struct CosmicTextColor color,
                 void (*draw_fn)(int32_t, int32_t, uint32_t, uint32_t, struct CosmicTextColor));

uint32_t buffer_new(struct Buffer **buffer,
                    struct FontSystem **font_system,
                    struct Metrics metrics);

void buffer_set_size(struct Buffer *buffer,
                     struct FontSystem *font_system,
                     float width,
                     float height);

void buffer_set_text(struct Buffer *buffer,
                     struct FontSystem *font_system,
                     const char *text,
                     struct Attrs attrs,
                     enum Shaping shaping);

void buffer_shape_until_scroll(struct Buffer *buffer, struct FontSystem *font_system, bool prune);

uint8_t color_a(struct CosmicTextColor color);

uint8_t color_b(struct CosmicTextColor color);

uint8_t color_g(struct CosmicTextColor color);

uint8_t color_r(struct CosmicTextColor color);

struct CosmicTextColor color_rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a);

uint32_t font_system_new(struct FontSystem **font_system);

struct Metrics metrics_new(float font_size, float line_height);

uint32_t swash_cache_new(struct SwashCache **swash_cache);

#endif /* cosmic_text_h */
