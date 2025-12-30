// Generated macro for test_gcs_service_account (function)
macro_rules! Depcrate_configtest_gcs_service_account {
() => {
// Module: crate::config
// Provides: {"test_gcs_service_account"}
// Dependencies: {}
# [test] # [serial] # [cfg (feature = "gcs")] fn test_gcs_service_account () { unsafe { env :: set_var ("SCCACHE_GCS_BUCKET" , "my-bucket") ; env :: set_var ("SCCACHE_GCS_SERVICE_ACCOUNT" , "my@example.com") ; env :: set_var ("SCCACHE_GCS_RW_MODE" , "READ_WRITE") ; } let cfg = config_from_env () ; unsafe { env :: remove_var ("SCCACHE_GCS_BUCKET") ; env :: remove_var ("SCCACHE_GCS_SERVICE_ACCOUNT") ; env :: remove_var ("SCCACHE_GCS_RW_MODE") ; } let env_cfg = cfg . unwrap () ; match env_cfg . cache . gcs { Some (GCSCacheConfig { ref bucket , service_account , rw_mode , .. }) => { assert_eq ! (bucket , "my-bucket") ; assert_eq ! (service_account , Some ("my@example.com" . to_string ())) ; assert_eq ! (rw_mode , CacheModeConfig :: ReadWrite) ; } None => unreachable ! () , } ; }
};
}
