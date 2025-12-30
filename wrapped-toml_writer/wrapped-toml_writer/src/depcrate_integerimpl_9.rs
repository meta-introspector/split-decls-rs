// Generated macro for impl_9 (impl)
macro_rules! Depcrate_integerimpl_9 {
() => {
// Module: crate::integer
// Provides: {"impl_9"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < u8 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
