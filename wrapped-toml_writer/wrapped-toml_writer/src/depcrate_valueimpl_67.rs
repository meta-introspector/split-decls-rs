// Generated macro for impl_67 (impl)
macro_rules! Depcrate_valueimpl_67 {
() => {
// Module: crate::value
// Provides: {"impl_67"}
// Dependencies: {}
impl WriteTomlValue for u64 { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write ! (writer , "{self}") } }
};
}
