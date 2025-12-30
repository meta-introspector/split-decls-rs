// Generated macro for impl_20 (impl)
macro_rules! Depcrate_integerimpl_20 {
() => {
// Module: crate::integer
// Provides: {"impl_20"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < isize > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
