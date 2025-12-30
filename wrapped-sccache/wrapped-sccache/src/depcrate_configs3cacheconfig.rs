// Generated macro for S3CacheConfig (struct)
macro_rules! Depcrate_configS3CacheConfig {
() => {
// Module: crate::config
// Provides: {"S3CacheConfig"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] # [serde (deny_unknown_fields)] pub struct S3CacheConfig { pub bucket : String , pub region : Option < String > , # [serde (default)] pub key_prefix : String , pub no_credentials : bool , pub endpoint : Option < String > , pub use_ssl : Option < bool > , pub server_side_encryption : Option < bool > , pub enable_virtual_host_style : Option < bool > , }
};
}
