// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl UnicodeWidthStr for str { # [inline] fn width (& self) -> usize { tables :: str_width (self) } # [cfg (feature = "cjk")] # [inline] fn width_cjk (& self) -> usize { tables :: str_width_cjk (self) } }
};
}
