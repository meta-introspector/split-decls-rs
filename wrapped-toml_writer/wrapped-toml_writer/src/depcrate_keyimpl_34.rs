// Generated macro for impl_34 (impl)
macro_rules! Depcrate_keyimpl_34 {
() => {
// Module: crate::key
// Provides: {"impl_34"}
// Dependencies: {}
impl < V : WriteTomlKey + ? Sized > WriteTomlKey for & V { fn write_toml_key < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { (* self) . write_toml_key (writer) } }
};
}
