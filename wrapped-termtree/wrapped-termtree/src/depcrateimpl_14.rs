// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < D : Display > Tree < D > { pub fn new (root : D) -> Self { Tree { root , leaves : Vec :: new () , multiline : false , glyphs : None , } } pub fn with_leaves (mut self , leaves : impl IntoIterator < Item = impl Into < Tree < D > > >) -> Self { self . leaves = leaves . into_iter () . map (Into :: into) . collect () ; self } # [doc = " Ensure all lines for `root` are indented"] pub fn with_multiline (mut self , yes : bool) -> Self { self . multiline = yes ; self } # [doc = " Customize the rendering of this node"] pub fn with_glyphs (mut self , glyphs : GlyphPalette) -> Self { self . glyphs = Some (glyphs) ; self } }
};
}
