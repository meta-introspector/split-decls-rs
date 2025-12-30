// Generated macro for DistToolchainConfig (enum)
macro_rules! Depcrate_configDistToolchainConfig {
() => {
// Module: crate::config
// Provides: {"DistToolchainConfig"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq , Serialize , Deserialize)] # [serde (deny_unknown_fields)] # [serde (tag = "type")] pub enum DistToolchainConfig { # [serde (rename = "no_dist")] NoDist { compiler_executable : PathBuf } , # [serde (rename = "path_override")] PathOverride { compiler_executable : PathBuf , archive : PathBuf , archive_compiler_executable : String , } , }
};
}
