// Generated macro for impl_61 (impl)
macro_rules! Depcrate_valueimpl_61 {
() => {
// Module: crate::value
// Provides: {"impl_61"}
// Dependencies: {}
impl WriteTomlValue for u8 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
};
}
