// Generated macro for impl_81 (impl)
macro_rules! Depcrate_encodeimpl_81 {
() => {
// Module: crate::encode
// Provides: {"impl_81"}
// Dependencies: {}
impl ValueRepr for i64 { fn to_repr (& self) -> Repr { let repr = self . to_toml_value () ; Repr :: new_unchecked (repr) } }
};
}
