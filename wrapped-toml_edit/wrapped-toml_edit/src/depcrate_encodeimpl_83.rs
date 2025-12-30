// Generated macro for impl_83 (impl)
macro_rules! Depcrate_encodeimpl_83 {
() => {
// Module: crate::encode
// Provides: {"impl_83"}
// Dependencies: {}
impl ValueRepr for bool { fn to_repr (& self) -> Repr { let repr = self . to_toml_value () ; Repr :: new_unchecked (repr) } }
};
}
