// Generated macro for impl_17 (impl)
macro_rules! Depcrate_integerimpl_17 {
() => {
// Module: crate::integer
// Provides: {"impl_17"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < u128 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
