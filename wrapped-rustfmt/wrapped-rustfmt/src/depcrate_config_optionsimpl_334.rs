// Generated macro for impl_334 (impl)
macro_rules! Depcrate_config_optionsimpl_334 {
() => {
// Module: crate::config::options
// Provides: {"impl_334"}
// Dependencies: {}
impl Color { # [doc = " Whether we should use a coloured terminal."] pub fn use_colored_tty (self) -> bool { match self { Color :: Always | Color :: Auto => true , Color :: Never => false , } } }
};
}
