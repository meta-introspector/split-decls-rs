// Generated macro for impl_1006 (impl)
macro_rules! Depcrate_configimpl_1006 {
() => {
// Module: crate::config
// Provides: {"impl_1006"}
// Dependencies: {}
impl CacheConfigs { # [doc = " Return cache type in an arbitrary but"] # [doc = " consistent ordering"] fn into_fallback (self) -> (Option < CacheType > , DiskCacheConfig) { let CacheConfigs { azure , disk , gcs , gha , memcached , redis , s3 , webdav , oss , } = self ; let cache_type = s3 . map (CacheType :: S3) . or_else (| | redis . map (CacheType :: Redis)) . or_else (| | memcached . map (CacheType :: Memcached)) . or_else (| | gcs . map (CacheType :: GCS)) . or_else (| | gha . map (CacheType :: GHA)) . or_else (| | azure . map (CacheType :: Azure)) . or_else (| | webdav . map (CacheType :: Webdav)) . or_else (| | oss . map (CacheType :: OSS)) ; let fallback = disk . unwrap_or_default () ; (cache_type , fallback) } # [doc = " Override self with any existing fields from other"] fn merge (& mut self , other : Self) { let CacheConfigs { azure , disk , gcs , gha , memcached , redis , s3 , webdav , oss , } = other ; if azure . is_some () { self . azure = azure ; } if disk . is_some () { self . disk = disk ; } if gcs . is_some () { self . gcs = gcs ; } if gha . is_some () { self . gha = gha ; } if memcached . is_some () { self . memcached = memcached ; } if redis . is_some () { self . redis = redis ; } if s3 . is_some () { self . s3 = s3 ; } if webdav . is_some () { self . webdav = webdav ; } if oss . is_some () { self . oss = oss ; } } }
};
}
