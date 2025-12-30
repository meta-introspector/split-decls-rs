// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl SpanVisitor < '_ > { fn put_span_prefix (& mut self) { if let Some (prefix) = self . field_prefix { self . buf . extend_from_slice (prefix . as_bytes ()) ; self . buf . push (b'_') ; } } }
};
}
