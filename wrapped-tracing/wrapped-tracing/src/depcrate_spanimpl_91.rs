// Generated macro for impl_91 (impl)
macro_rules! Depcrate_spanimpl_91 {
() => {
// Module: crate::span
// Provides: {"impl_91"}
// Dependencies: {}
impl < 'a > From < & 'a Span > for Option < Id > { fn from (span : & 'a Span) -> Self { span . inner . as_ref () . map (Inner :: id) } }
};
}
