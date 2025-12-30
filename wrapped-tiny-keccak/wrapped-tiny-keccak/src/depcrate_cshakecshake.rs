// Generated macro for CShake (struct)
macro_rules! Depcrate_cshakeCShake {
() => {
// Module: crate::cshake
// Provides: {"CShake"}
// Dependencies: {}
# [doc = " The `cSHAKE` extendable-output functions defined in [`SP800-185`]."] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " tiny-keccak = { version = \"2.0.0\", features = [\"cshake\"] }"] # [doc = " ```"] # [doc = ""] # [doc = " [`SP800-185`]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf"] # [derive (Clone)] pub struct CShake { state : KeccakState < KeccakF > , }
};
}
