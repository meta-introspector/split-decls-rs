// Generated macro for impl_27 (impl)
macro_rules! Depcrate_spanimpl_27 {
() => {
// Module: crate::span
// Provides: {"impl_27"}
// Dependencies: {}
impl From < Span > for miette :: SourceSpan { fn from (value : Span) -> Self { let start = value . 0 ; let len = value . 1 - start ; (start , len) . into () } }
};
}
