// Generated macro for impl_64 (impl)
macro_rules! Depcrate_valueimpl_64 {
() => {
// Module: crate::value
// Provides: {"impl_64"}
// Dependencies: {}
impl WriteTomlValue for i16 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
};
}
