// Generated macro for tests (module)
macro_rules! Depcrate_crypto_aws_lc_rstests {
() => {
// Module: crate::crypto::aws_lc_rs
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [cfg (feature = "fips")] # [test] fn default_suites_are_fips () { assert ! (super :: DEFAULT_TLS12_CIPHER_SUITES . iter () . all (| scs | scs . fips ())) ; assert ! (super :: DEFAULT_TLS13_CIPHER_SUITES . iter () . all (| scs | scs . fips ())) ; } # [cfg (not (feature = "fips"))] # [test] fn default_suites () { assert_eq ! (super :: DEFAULT_TLS12_CIPHER_SUITES , super :: ALL_TLS12_CIPHER_SUITES) ; assert_eq ! (super :: DEFAULT_TLS13_CIPHER_SUITES , super :: ALL_TLS13_CIPHER_SUITES) ; } }
};
}
