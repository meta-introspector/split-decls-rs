// Generated macro for MemcachedCacheConfig (struct)
macro_rules! Depcrate_configMemcachedCacheConfig {
() => {
// Module: crate::config
// Provides: {"MemcachedCacheConfig"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Serialize , Deserialize , Default)] # [serde (deny_unknown_fields)] pub struct MemcachedCacheConfig { # [serde (alias = "endpoint")] pub url : String , # [doc = " Username to authenticate with."] pub username : Option < String > , # [doc = " Password to authenticate with."] pub password : Option < String > , # [doc = " the expiration time in seconds."] # [doc = ""] # [doc = " Default to 24 hours (86400)"] # [doc = " Up to 30 days (2592000)"] # [serde (default = "default_memcached_cache_expiration")] pub expiration : u32 , # [serde (default)] pub key_prefix : String , }
};
}
