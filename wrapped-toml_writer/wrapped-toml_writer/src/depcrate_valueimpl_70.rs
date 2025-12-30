// Generated macro for impl_70 (impl)
macro_rules! Depcrate_valueimpl_70 {
() => {
// Module: crate::value
// Provides: {"impl_70"}
// Dependencies: {}
impl WriteTomlValue for i128 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
};
}
