// Generated macro for WebdavCacheConfig (struct)
macro_rules! Depcrate_configWebdavCacheConfig {
() => {
// Module: crate::config
// Provides: {"WebdavCacheConfig"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] # [serde (deny_unknown_fields)] pub struct WebdavCacheConfig { pub endpoint : String , # [serde (default)] pub key_prefix : String , pub username : Option < String > , pub password : Option < String > , pub token : Option < String > , }
};
}
