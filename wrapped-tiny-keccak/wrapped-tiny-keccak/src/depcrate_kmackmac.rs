// Generated macro for Kmac (struct)
macro_rules! Depcrate_kmacKmac {
() => {
// Module: crate::kmac
// Provides: {"Kmac"}
// Dependencies: {}
# [doc = " The `KMAC` pseudo-random functions defined in [`SP800-185`]."] # [doc = ""] # [doc = " The KECCAK Message Authentication Code (`KMAC`) algorithm is a `PRF` and keyed hash function based"] # [doc = " on KECCAK. It provides variable-length output, and unlike [`SHAKE`] and [`cSHAKE`], altering the"] # [doc = " requested output length generates a new, unrelated output. KMAC has two variants, [`KMAC128`] and"] # [doc = " [`KMAC256`], built from [`cSHAKE128`] and [`cSHAKE256`], respectively. The two variants differ somewhat in"] # [doc = " their technical security properties."] # [doc = ""] # [doc = " # Usage"] # [doc = ""] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " tiny-keccak = { version = \"2.0.0\", features = [\"kmac\"] }"] # [doc = " ```"] # [doc = ""] # [doc = " [`SP800-185`]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf"] # [doc = " [`KMAC128`]: struct.Kmac.html#method.v128"] # [doc = " [`KMAC256`]: struct.Kmac.html#method.v256"] # [doc = " [`SHAKE`]: struct.Shake.html"] # [doc = " [`cSHAKE`]: struct.CShake.html"] # [doc = " [`cSHAKE128`]: struct.CShake.html#method.v128"] # [doc = " [`cSHAKE256`]: struct.CShake.html#method.v256"] # [derive (Clone)] pub struct Kmac { state : CShake , }
};
}
