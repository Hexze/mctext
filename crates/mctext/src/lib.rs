mod color;
pub mod fonts;
mod json;
mod style;
mod text;

#[cfg(feature = "render")]
mod layout;
#[cfg(feature = "render")]
mod render;
#[cfg(feature = "render")]
mod system;

pub use color::{NamedColor, SHADOW_OFFSET, TextColor, shadow_color};
pub use fonts::{FontFamily, FontVariant, FontVersion};

#[cfg(feature = "modern-fonts")]
pub use fonts::{MINECRAFT_BOLD, MINECRAFT_BOLD_ITALIC, MINECRAFT_ITALIC, MINECRAFT_REGULAR};

#[cfg(feature = "legacy-fonts")]
pub use fonts::{LEGACY_BOLD, LEGACY_BOLD_ITALIC, LEGACY_ITALIC, LEGACY_REGULAR};

#[cfg(feature = "special-fonts")]
pub use fonts::{ENCHANTING_REGULAR, ILLAGER_REGULAR};

pub use json::{ParseError, to_json, try_parse_json_component};
pub use style::Style;
pub use text::{MCText, Span, SpanBuilder, count_visible_chars, strip_codes};

#[cfg(feature = "render")]
pub use layout::{LayoutEngine, LayoutOptions, PositionedGlyph, TextAlign, TextLayout};
#[cfg(feature = "render")]
pub use render::{RasterizedGlyph, SoftwareRenderer, TextRenderContext, TextRenderer};
#[cfg(feature = "render")]
pub use system::{FontSystem, GlyphMetrics};
