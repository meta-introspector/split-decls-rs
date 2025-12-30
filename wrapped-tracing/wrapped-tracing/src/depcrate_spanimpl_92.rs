// Generated macro for impl_92 (impl)
macro_rules! Depcrate_spanimpl_92 {
() => {
// Module: crate::span
// Provides: {"impl_92"}
// Dependencies: {}
impl From < Span > for Option < Id > { fn from (span : Span) -> Self { span . inner . as_ref () . map (Inner :: id) } }
};
}
