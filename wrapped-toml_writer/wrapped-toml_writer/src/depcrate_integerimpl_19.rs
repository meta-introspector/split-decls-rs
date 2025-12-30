// Generated macro for impl_19 (impl)
macro_rules! Depcrate_integerimpl_19 {
() => {
// Module: crate::integer
// Provides: {"impl_19"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < usize > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
