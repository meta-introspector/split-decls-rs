// Generated macro for impl_23 (impl)
macro_rules! Depcrate_deimpl_23 {
() => {
// Module: crate::de
// Provides: {"impl_23"}
// Dependencies: {}
impl From < de :: value :: Error > for DeserializerError { fn from (e : de :: value :: Error) -> DeserializerError { DeserializerError :: Custom (e . to_string ()) } }
};
}
