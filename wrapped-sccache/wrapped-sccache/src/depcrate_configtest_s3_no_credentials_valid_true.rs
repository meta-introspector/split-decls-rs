// Generated macro for test_s3_no_credentials_valid_true (function)
macro_rules! Depcrate_configtest_s3_no_credentials_valid_true {
() => {
// Module: crate::config
// Provides: {"test_s3_no_credentials_valid_true"}
// Dependencies: {}
# [test] # [serial] fn test_s3_no_credentials_valid_true () { unsafe { env :: set_var ("SCCACHE_S3_NO_CREDENTIALS" , "true") ; env :: set_var ("SCCACHE_BUCKET" , "my-bucket") ; } let cfg = config_from_env () ; unsafe { env :: remove_var ("SCCACHE_S3_NO_CREDENTIALS") ; env :: remove_var ("SCCACHE_BUCKET") ; } let env_cfg = cfg . unwrap () ; match env_cfg . cache . s3 { Some (S3CacheConfig { ref bucket , no_credentials , .. }) => { assert_eq ! (bucket , "my-bucket") ; assert ! (no_credentials) ; } None => unreachable ! () , } ; }
};
}
