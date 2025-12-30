// Generated macro for impl_1499 (impl)
macro_rules! Depcrate_testimpl_1499 {
() => {
// Module: crate::test
// Provides: {"impl_1499"}
// Dependencies: {}
impl < 'a > Iterator for CharsIgnoreNewlineRepr < 'a > { type Item = char ; fn next (& mut self) -> Option < char > { self . 0 . next () . map (| c | { if c == '\r' { if * self . 0 . peek () . unwrap_or (& '\0') == '\n' { self . 0 . next () ; '\n' } else { '\r' } } else { c } }) } }
};
}
