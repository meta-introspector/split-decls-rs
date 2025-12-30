// Generated macro for test_s3_no_credentials_conflict (function)
macro_rules! Depcrate_configtest_s3_no_credentials_conflict {
() => {
// Module: crate::config
// Provides: {"test_s3_no_credentials_conflict"}
// Dependencies: {}
# [test] # [serial] # [cfg (feature = "s3")] fn test_s3_no_credentials_conflict () { unsafe { env :: set_var ("SCCACHE_S3_NO_CREDENTIALS" , "true") ; env :: set_var ("SCCACHE_BUCKET" , "my-bucket") ; env :: set_var ("AWS_ACCESS_KEY_ID" , "aws-access-key-id") ; env :: set_var ("AWS_SECRET_ACCESS_KEY" , "aws-secret-access-key") ; } let cfg = config_from_env () ; unsafe { env :: remove_var ("SCCACHE_S3_NO_CREDENTIALS") ; env :: remove_var ("SCCACHE_BUCKET") ; env :: remove_var ("AWS_ACCESS_KEY_ID") ; env :: remove_var ("AWS_SECRET_ACCESS_KEY") ; } let error = cfg . unwrap_err () ; assert_eq ! ("If setting S3 credentials, SCCACHE_S3_NO_CREDENTIALS must not be set." , error . to_string ()) ; }
};
}
