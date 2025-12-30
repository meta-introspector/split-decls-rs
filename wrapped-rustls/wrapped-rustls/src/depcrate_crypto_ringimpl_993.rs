// Generated macro for impl_993 (impl)
macro_rules! Depcrate_crypto_ringimpl_993 {
() => {
// Module: crate::crypto::ring
// Provides: {"impl_993"}
// Dependencies: {}
impl SecureRandom for Ring { fn fill (& self , buf : & mut [u8]) -> Result < () , GetRandomFailed > { use ring :: rand :: SecureRandom ; ring :: rand :: SystemRandom :: new () . fill (buf) . map_err (| _ | GetRandomFailed) } }
};
}
