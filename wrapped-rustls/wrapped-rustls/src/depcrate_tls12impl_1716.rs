// Generated macro for impl_1716 (impl)
macro_rules! Depcrate_tls12impl_1716 {
() => {
// Module: crate::tls12
// Provides: {"impl_1716"}
// Dependencies: {}
impl Tls12CipherSuite { # [doc = " Resolve the set of supported [`SignatureScheme`]s from the"] # [doc = " offered signature schemes.  If we return an empty"] # [doc = " set, the handshake terminates."] pub fn resolve_sig_schemes (& self , offered : & [SignatureScheme]) -> Vec < SignatureScheme > { self . sign . iter () . filter (| pref | offered . contains (pref)) . copied () . collect () } # [doc = " Return `true` if this is backed by a FIPS-approved implementation."] # [doc = ""] # [doc = " This means all the constituent parts that do cryptography return `true` for `fips()`."] pub fn fips (& self) -> bool { self . common . fips () && self . prf_provider . fips () && self . aead_alg . fips () } }
};
}
