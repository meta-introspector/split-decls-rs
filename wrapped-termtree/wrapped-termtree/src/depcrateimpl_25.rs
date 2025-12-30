// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl GlyphPalette { pub const fn new () -> Self { Self { middle_item : "├" , last_item : "└" , item_indent : "── " , middle_skip : "│" , last_skip : " " , skip_indent : "   " , } } fn middle_space (& self) -> SpacePalette { SpacePalette { skip : self . middle_skip , indent : self . skip_indent , } } fn last_space (& self) -> SpacePalette { SpacePalette { skip : self . last_skip , indent : self . skip_indent , } } }
};
}
