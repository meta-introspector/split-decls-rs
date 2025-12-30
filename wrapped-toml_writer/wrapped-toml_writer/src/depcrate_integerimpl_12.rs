// Generated macro for impl_12 (impl)
macro_rules! Depcrate_integerimpl_12 {
() => {
// Module: crate::integer
// Provides: {"impl_12"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < i16 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
