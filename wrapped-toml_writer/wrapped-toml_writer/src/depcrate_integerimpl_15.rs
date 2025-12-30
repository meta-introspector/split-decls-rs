// Generated macro for impl_15 (impl)
macro_rules! Depcrate_integerimpl_15 {
() => {
// Module: crate::integer
// Provides: {"impl_15"}
// Dependencies: {}
impl crate :: WriteTomlValue for TomlInteger < u64 > { fn write_toml_value < W : crate :: TomlWrite + ? Sized > (& self , writer : & mut W) -> fmt :: Result { write_toml_value (self . value , & self . format , writer) } }
};
}
