// Generated macro for FileConfig (struct)
macro_rules! Depcrate_configFileConfig {
() => {
// Module: crate::config
// Provides: {"FileConfig"}
// Dependencies: {}
# [derive (Debug , Default , Serialize , Deserialize , Eq , PartialEq)] # [serde (default)] # [serde (deny_unknown_fields)] pub struct FileConfig { pub cache : CacheConfigs , pub dist : DistConfig , pub server_startup_timeout_ms : Option < u64 > , }
};
}
