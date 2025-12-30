// Generated macro for fips (function)
macro_rules! Depcrate_crypto_aws_lc_rsfips {
() => {
// Module: crate::crypto::aws_lc_rs
// Provides: {"fips"}
// Dependencies: {}
# [doc = " Are we in FIPS mode?"] pub (super) fn fips () -> bool { aws_lc_rs :: try_fips_mode () . is_ok () }
};
}
