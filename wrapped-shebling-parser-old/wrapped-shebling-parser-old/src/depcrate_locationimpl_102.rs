// Generated macro for impl_102 (impl)
macro_rules! Depcrate_locationimpl_102 {
() => {
// Module: crate::location
// Provides: {"impl_102"}
// Dependencies: {}
impl Location { # [doc = " Returns the location's line."] pub (crate) fn line (& self) -> u32 { self . line } # [doc = " Returns the location's column."] pub (crate) fn column (& self) -> usize { self . column } # [doc = " Translates the location by the given offset."] # [doc = ""] # [doc = " This method won't panic, since it saturates at the numeric bounds"] # [doc = " instead of overflowing."] pub (crate) fn translate (mut self , offset : isize) -> Self { self . offset = self . offset . saturating_add_signed (offset) ; self . column = self . column . saturating_add_signed (offset) ; self } }
};
}
