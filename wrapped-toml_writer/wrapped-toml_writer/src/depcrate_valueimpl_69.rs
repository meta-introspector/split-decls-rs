// Generated macro for impl_69 (impl)
macro_rules! Depcrate_valueimpl_69 {
() => {
// Module: crate::value
// Provides: {"impl_69"}
// Dependencies: {}
impl WriteTomlValue for u128 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
};
}
