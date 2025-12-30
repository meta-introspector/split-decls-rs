// Generated macro for impl_11 (impl)
macro_rules! Depcrate_integerimpl_11 {
() => {
// Module: crate::integer
// Provides: {"impl_11"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < u16 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
