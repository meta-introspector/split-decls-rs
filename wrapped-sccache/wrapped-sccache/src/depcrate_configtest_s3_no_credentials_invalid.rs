// Generated macro for test_s3_no_credentials_invalid (function)
macro_rules! Depcrate_configtest_s3_no_credentials_invalid {
() => {
// Module: crate::config
// Provides: {"test_s3_no_credentials_invalid"}
// Dependencies: {}
# [test] # [serial] fn test_s3_no_credentials_invalid () { unsafe { env :: set_var ("SCCACHE_S3_NO_CREDENTIALS" , "yes") ; env :: set_var ("SCCACHE_BUCKET" , "my-bucket") ; } let cfg = config_from_env () ; unsafe { env :: remove_var ("SCCACHE_S3_NO_CREDENTIALS") ; env :: remove_var ("SCCACHE_BUCKET") ; } let error = cfg . unwrap_err () ; assert_eq ! ("SCCACHE_S3_NO_CREDENTIALS must be 'true', 'on', '1', 'false', 'off' or '0'." , error . to_string ()) ; }
};
}
