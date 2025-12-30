// Generated macro for impl_135 (impl)
macro_rules! Depcrate_format_descriptionimpl_135 {
() => {
// Module: crate::format_description
// Provides: {"impl_135"}
// Dependencies: {}
impl From < Error > for crate :: Error { fn from (error : Error) -> Self { Self :: Custom { message : error . message . into () , span_start : Some (error . proc_span) , span_end : Some (error . proc_span) , } } }
};
}
