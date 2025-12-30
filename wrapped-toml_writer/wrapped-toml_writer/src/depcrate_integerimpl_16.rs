// Generated macro for impl_16 (impl)
macro_rules! Depcrate_integerimpl_16 {
() => {
// Module: crate::integer
// Provides: {"impl_16"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < i64 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
