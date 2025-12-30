// Generated macro for impl_83 (impl)
macro_rules! Depcrate_valueimpl_83 {
() => {
// Module: crate::value
// Provides: {"impl_83"}
// Dependencies: {}
impl < V : WriteTomlValue + ? Sized > WriteTomlValue for & V { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { (* self) . write_toml_value (writer) } }
};
}
