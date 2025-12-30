// Generated macro for impl_10 (impl)
macro_rules! Depcrate_integerimpl_10 {
() => {
// Module: crate::integer
// Provides: {"impl_10"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < i8 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
