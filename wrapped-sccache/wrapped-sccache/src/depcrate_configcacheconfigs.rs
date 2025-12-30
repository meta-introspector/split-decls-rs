// Generated macro for CacheConfigs (struct)
macro_rules! Depcrate_configCacheConfigs {
() => {
// Module: crate::config
// Provides: {"CacheConfigs"}
// Dependencies: {}
# [derive (Debug , Default , Serialize , Deserialize , PartialEq , Eq)] # [serde (deny_unknown_fields)] pub struct CacheConfigs { pub azure : Option < AzureCacheConfig > , pub disk : Option < DiskCacheConfig > , pub gcs : Option < GCSCacheConfig > , pub gha : Option < GHACacheConfig > , pub memcached : Option < MemcachedCacheConfig > , pub redis : Option < RedisCacheConfig > , pub s3 : Option < S3CacheConfig > , pub webdav : Option < WebdavCacheConfig > , pub oss : Option < OSSCacheConfig > , }
};
}
