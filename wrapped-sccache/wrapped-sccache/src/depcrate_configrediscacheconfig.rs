// Generated macro for RedisCacheConfig (struct)
macro_rules! Depcrate_configRedisCacheConfig {
() => {
// Module: crate::config
// Provides: {"RedisCacheConfig"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Serialize , Deserialize , Default)] # [serde (deny_unknown_fields)] pub struct RedisCacheConfig { # [doc = " The single-node redis endpoint."] # [doc = " Mutually exclusive with `cluster_endpoints`."] pub endpoint : Option < String > , # [doc = " The redis cluster endpoints."] # [doc = " Mutually exclusive with `endpoint`."] pub cluster_endpoints : Option < String > , # [doc = " Username to authenticate with."] pub username : Option < String > , # [doc = " Password to authenticate with."] pub password : Option < String > , # [doc = " The redis URL."] # [doc = " Deprecated in favor of `endpoint`."] pub url : Option < String > , # [doc = " the db number to use"] # [doc = ""] # [doc = " Default to 0"] # [serde (default)] pub db : u32 , # [doc = " the ttl (expiration) time in seconds."] # [doc = ""] # [doc = " Default to infinity (0)"] # [serde (default , alias = "expiration")] pub ttl : u64 , # [serde (default)] pub key_prefix : String , }
};
}
