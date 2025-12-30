// Generated macro for impl_13 (impl)
macro_rules! Depcrate_attrimpl_13 {
() => {
// Module: crate::attr
// Provides: {"impl_13"}
// Dependencies: {}
impl Position { # [allow (clippy :: trivially_copy_pass_by_ref)] fn as_str (& self) -> & 'static str { match self { Position :: Struct => "struct" , Position :: Enum => "enum" , Position :: Variant => "variant" , Position :: NamedField => "named field" , Position :: UnnamedField => "unnamed field" , } } fn is_field (self) -> bool { self == Position :: NamedField || self == Position :: UnnamedField } }
};
}
