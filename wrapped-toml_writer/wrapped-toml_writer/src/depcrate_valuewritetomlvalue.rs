// Generated macro for WriteTomlValue (trait)
macro_rules! Depcrate_valueWriteTomlValue {
() => {
// Module: crate::value
// Provides: {"WriteTomlValue"}
// Dependencies: {}
pub trait WriteTomlValue { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result ; }
};
}
