// Generated macro for impl_1954 (impl)
macro_rules! Depcrate_suitesimpl_1954 {
() => {
// Module: crate::suites
// Provides: {"impl_1954"}
// Dependencies: {}
impl CipherSuiteCommon { # [doc = " Return `true` if this is backed by a FIPS-approved implementation."] # [doc = ""] # [doc = " This means all the constituent parts that do cryptography return `true` for `fips()`."] pub fn fips (& self) -> bool { self . hash_provider . fips () } }
};
}
