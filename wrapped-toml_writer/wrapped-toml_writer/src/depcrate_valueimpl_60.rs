// Generated macro for impl_60 (impl)
macro_rules! Depcrate_valueimpl_60 {
() => {
// Module: crate::value
// Provides: {"impl_60"}
// Dependencies: {}
impl WriteTomlValue for bool { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
};
}
