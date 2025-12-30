// Generated macro for impl_388 (impl)
macro_rules! Depcrate_config_file_linesimpl_388 {
() => {
// Module: crate::config::file_lines
// Provides: {"impl_388"}
// Dependencies: {}
impl < 'a > iter :: Iterator for Files < 'a > { type Item = & 'a FileName ; fn next (& mut self) -> Option < & 'a FileName > { self . 0 . as_mut () . and_then (Iterator :: next) } }
};
}
