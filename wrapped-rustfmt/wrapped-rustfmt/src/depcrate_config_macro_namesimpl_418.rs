// Generated macro for impl_418 (impl)
macro_rules! Depcrate_config_macro_namesimpl_418 {
() => {
// Module: crate::config::macro_names
// Provides: {"impl_418"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for MacroSelector { fn deserialize < D > (de : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let s = String :: deserialize (de) ? ; std :: str :: FromStr :: from_str (& s) . map_err (serde :: de :: Error :: custom) } }
};
}
