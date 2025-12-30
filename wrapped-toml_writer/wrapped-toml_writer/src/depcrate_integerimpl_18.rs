// Generated macro for impl_18 (impl)
macro_rules! Depcrate_integerimpl_18 {
() => {
// Module: crate::integer
// Provides: {"impl_18"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < i128 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
