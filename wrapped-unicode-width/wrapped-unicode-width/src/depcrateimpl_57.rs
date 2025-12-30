// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl UnicodeWidthChar for char { # [inline] fn width (self) -> Option < usize > { tables :: single_char_width (self) } # [cfg (feature = "cjk")] # [inline] fn width_cjk (self) -> Option < usize > { tables :: single_char_width_cjk (self) } }
};
}
