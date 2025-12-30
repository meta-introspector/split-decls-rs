// Generated macro for impl_1337 (impl)
macro_rules! Depcrate_crypto_aws_lc_rsimpl_1337 {
() => {
// Module: crate::crypto::aws_lc_rs
// Provides: {"impl_1337"}
// Dependencies: {}
impl SecureRandom for AwsLcRs { fn fill (& self , buf : & mut [u8]) -> Result < () , GetRandomFailed > { use aws_lc_rs :: rand :: SecureRandom ; aws_lc_rs :: rand :: SystemRandom :: new () . fill (buf) . map_err (| _ | GetRandomFailed) } fn fips (& self) -> bool { fips () } }
};
}
