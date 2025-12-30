// Generated macro for DistConfig (struct)
macro_rules! Depcrate_configDistConfig {
() => {
// Module: crate::config
// Provides: {"DistConfig"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] # [serde (default)] # [serde (deny_unknown_fields)] pub struct DistConfig { pub auth : DistAuth , # [cfg (any (feature = "dist-client" , feature = "dist-server"))] pub scheduler_url : Option < HTTPUrl > , # [cfg (not (any (feature = "dist-client" , feature = "dist-server")))] pub scheduler_url : Option < String > , pub cache_dir : PathBuf , pub toolchains : Vec < DistToolchainConfig > , # [serde (deserialize_with = "deserialize_size_from_str")] pub toolchain_cache_size : u64 , pub rewrite_includes_only : bool , }
};
}
