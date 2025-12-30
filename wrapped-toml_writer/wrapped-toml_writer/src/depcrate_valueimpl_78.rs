// Generated macro for impl_78 (impl)
macro_rules! Depcrate_valueimpl_78 {
() => {
// Module: crate::value
// Provides: {"impl_78"}
// Dependencies: {}
impl < V : WriteTomlValue , const N : usize > WriteTomlValue for [V ; N] { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { self . as_slice () . write_toml_value (writer) } }
};
}
