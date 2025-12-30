// Generated macro for impl_93 (impl)
macro_rules! Depcrate_spanimpl_93 {
() => {
// Module: crate::span
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'a > From < & 'a EnteredSpan > for Option < & 'a Id > { fn from (span : & 'a EnteredSpan) -> Self { span . inner . as_ref () . map (| inner | & inner . id) } }
};
}
