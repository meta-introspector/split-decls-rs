// Generated macro for impl_65 (impl)
macro_rules! Depcrate_valueimpl_65 {
() => {
// Module: crate::value
// Provides: {"impl_65"}
// Dependencies: {}
impl WriteTomlValue for u32 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
};
}
