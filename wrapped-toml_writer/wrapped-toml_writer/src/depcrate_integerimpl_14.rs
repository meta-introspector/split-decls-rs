// Generated macro for impl_14 (impl)
macro_rules! Depcrate_integerimpl_14 {
() => {
// Module: crate::integer
// Provides: {"impl_14"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < i32 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
