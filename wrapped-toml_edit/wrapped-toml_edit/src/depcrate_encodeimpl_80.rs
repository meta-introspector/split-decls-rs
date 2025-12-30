// Generated macro for impl_80 (impl)
macro_rules! Depcrate_encodeimpl_80 {
() => {
// Module: crate::encode
// Provides: {"impl_80"}
// Dependencies: {}
impl ValueRepr for String { fn to_repr (& self) -> Repr { let output = toml_writer :: TomlStringBuilder :: new (self . as_str ()) . as_default () . to_toml_value () ; Repr :: new_unchecked (output) } }
};
}
