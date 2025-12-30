// Generated macro for OSSCacheConfig (struct)
macro_rules! Depcrate_configOSSCacheConfig {
() => {
// Module: crate::config
// Provides: {"OSSCacheConfig"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] # [serde (deny_unknown_fields)] pub struct OSSCacheConfig { pub bucket : String , # [serde (default)] pub key_prefix : String , pub endpoint : Option < String > , pub no_credentials : bool , }
};
}
