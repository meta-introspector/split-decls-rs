// Generated macro for GHACacheConfig (struct)
macro_rules! Depcrate_configGHACacheConfig {
() => {
// Module: crate::config
// Provides: {"GHACacheConfig"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] # [serde (deny_unknown_fields)] pub struct GHACacheConfig { pub enabled : bool , # [doc = " Version for gha cache is a namespace. By setting different versions,"] # [doc = " we can avoid mixed caches."] pub version : String , }
};
}
