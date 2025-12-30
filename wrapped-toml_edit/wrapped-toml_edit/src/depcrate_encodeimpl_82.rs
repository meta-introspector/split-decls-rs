// Generated macro for impl_82 (impl)
macro_rules! Depcrate_encodeimpl_82 {
() => {
// Module: crate::encode
// Provides: {"impl_82"}
// Dependencies: {}
impl ValueRepr for f64 { fn to_repr (& self) -> Repr { let repr = self . to_toml_value () ; Repr :: new_unchecked (repr) } }
};
}
