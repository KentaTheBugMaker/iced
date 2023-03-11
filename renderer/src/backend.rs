use crate::core::text;
use crate::core::{Font, Point, Size};
use crate::graphics::backend;

use std::borrow::Cow;

#[allow(clippy::large_enum_variant)]
pub enum Backend {
    #[cfg(feature = "wgpu")]
    Wgpu(iced_wgpu::Backend),
    #[cfg(feature = "tiny-skia")]
    TinySkia(iced_tiny_skia::Backend),
}

macro_rules! delegate {
    ($backend:expr, $name:ident, $body:expr) => {
        match $backend {
            #[cfg(feature = "wgpu")]
            Self::Wgpu($name) => $body,
            #[cfg(feature = "tiny-skia")]
            Self::TinySkia($name) => $body,
        }
    };
}

impl iced_graphics::Backend for Backend {
    fn trim_measurements(&mut self) {
        delegate!(self, backend, backend.trim_measurements());
    }
}

impl backend::Text for Backend {
    const ICON_FONT: Font = Font::Name("Iced-Icons");
    const CHECKMARK_ICON: char = '\u{f00c}';
    const ARROW_DOWN_ICON: char = '\u{e800}';

    fn default_font(&self) -> Font {
        delegate!(self, backend, backend.default_font())
    }

    fn default_size(&self) -> f32 {
        delegate!(self, backend, backend.default_size())
    }

    fn measure(
        &self,
        contents: &str,
        size: f32,
        font: Font,
        bounds: Size,
    ) -> (f32, f32) {
        delegate!(self, backend, backend.measure(contents, size, font, bounds))
    }

    fn hit_test(
        &self,
        contents: &str,
        size: f32,
        font: Font,
        bounds: Size,
        position: Point,
        nearest_only: bool,
    ) -> Option<text::Hit> {
        delegate!(
            self,
            backend,
            backend.hit_test(
                contents,
                size,
                font,
                bounds,
                position,
                nearest_only
            )
        )
    }

    fn load_font(&mut self, font: Cow<'static, [u8]>) {
        delegate!(self, backend, backend.load_font(font));
    }
}

#[cfg(feature = "image")]
impl backend::Image for Backend {
    fn dimensions(&self, handle: &crate::core::image::Handle) -> Size<u32> {
        delegate!(self, backend, backend.dimensions(handle))
    }
}

#[cfg(feature = "svg")]
impl backend::Svg for Backend {
    fn viewport_dimensions(
        &self,
        handle: &crate::core::svg::Handle,
    ) -> Size<u32> {
        delegate!(self, backend, backend.viewport_dimensions(handle))
    }
}
