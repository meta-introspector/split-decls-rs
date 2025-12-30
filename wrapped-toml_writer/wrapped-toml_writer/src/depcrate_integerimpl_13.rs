// Generated macro for impl_13 (impl)
macro_rules! Depcrate_integerimpl_13 {
() => {
// Module: crate::integer
// Provides: {"impl_13"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < u32 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
