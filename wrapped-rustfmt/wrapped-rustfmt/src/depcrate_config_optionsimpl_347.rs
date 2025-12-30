// Generated macro for impl_347 (impl)
macro_rules! Depcrate_config_optionsimpl_347 {
() => {
// Module: crate::config::options
// Provides: {"impl_347"}
// Dependencies: {}
impl IgnoreList { pub fn add_prefix (& mut self , dir : & Path) { self . rustfmt_toml_path = dir . to_path_buf () ; } pub fn rustfmt_toml_path (& self) -> & Path { & self . rustfmt_toml_path } }
};
}
