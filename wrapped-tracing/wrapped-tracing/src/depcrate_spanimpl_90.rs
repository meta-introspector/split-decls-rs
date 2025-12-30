// Generated macro for impl_90 (impl)
macro_rules! Depcrate_spanimpl_90 {
() => {
// Module: crate::span
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a > From < & 'a Span > for Option < & 'a Id > { fn from (span : & 'a Span) -> Self { span . inner . as_ref () . map (| inner | & inner . id) } }
};
}
