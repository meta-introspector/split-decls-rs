// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < D : Display > Tree < D > { # [doc = " Ensure all lines for `root` are indented"] pub fn set_multiline (& mut self , yes : bool) -> & mut Self { self . multiline = yes ; self } # [doc = " Customize the rendering of this node"] pub fn set_glyphs (& mut self , glyphs : GlyphPalette) -> & mut Self { self . glyphs = Some (glyphs) ; self } }
};
}
