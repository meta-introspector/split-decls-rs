// Generated macro for CacheType (enum)
macro_rules! Depcrate_configCacheType {
() => {
// Module: crate::config
// Provides: {"CacheType"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq)] pub enum CacheType { Azure (AzureCacheConfig) , GCS (GCSCacheConfig) , GHA (GHACacheConfig) , Memcached (MemcachedCacheConfig) , Redis (RedisCacheConfig) , S3 (S3CacheConfig) , Webdav (WebdavCacheConfig) , OSS (OSSCacheConfig) , }
};
}
