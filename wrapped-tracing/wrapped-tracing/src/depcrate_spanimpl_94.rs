// Generated macro for impl_94 (impl)
macro_rules! Depcrate_spanimpl_94 {
() => {
// Module: crate::span
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'a > From < & 'a EnteredSpan > for Option < Id > { fn from (span : & 'a EnteredSpan) -> Self { span . inner . as_ref () . map (Inner :: id) } }
};
}
