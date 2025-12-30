// Generated macro for impl_38 (impl)
macro_rules! Depcrate_patch_configimpl_38 {
() => {
// Module: crate::patch_config
// Provides: {"impl_38"}
// Dependencies: {}
impl PatchConfig { pub fn load_from_file (path : & std :: path :: Path) -> anyhow :: Result < Self > { if ! path . exists () { anyhow :: bail ! ("Patch config file not found at {}" , path . display ()) ; } let content = std :: fs :: read_to_string (path) ? ; let config : Self = toml :: from_str (& content) ? ; Ok (config) } }
};
}
