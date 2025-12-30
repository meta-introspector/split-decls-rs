// Generated macro for GCSCacheConfig (struct)
macro_rules! Depcrate_configGCSCacheConfig {
() => {
// Module: crate::config
// Provides: {"GCSCacheConfig"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] # [serde (deny_unknown_fields)] pub struct GCSCacheConfig { pub bucket : String , pub key_prefix : String , pub cred_path : Option < String > , pub service_account : Option < String > , pub rw_mode : CacheModeConfig , pub credential_url : Option < String > , }
};
}
