// Generated macro for impl_1793 (impl)
macro_rules! Depcrate_tls13impl_1793 {
() => {
// Module: crate::tls13
// Provides: {"impl_1793"}
// Dependencies: {}
impl Tls13CipherSuite { # [doc = " Can a session using suite self resume from suite prev?"] pub fn can_resume_from (& self , prev : & 'static Self) -> Option < & 'static Self > { (prev . common . hash_provider . algorithm () == self . common . hash_provider . algorithm ()) . then_some (prev) } # [doc = " Return `true` if this is backed by a FIPS-approved implementation."] # [doc = ""] # [doc = " This means all the constituent parts that do cryptography return `true` for `fips()`."] pub fn fips (& self) -> bool { let Self { common , protocol_version : _ , hkdf_provider , aead_alg , quic , } = self ; common . fips () && hkdf_provider . fips () && aead_alg . fips () && quic . map (| q | q . fips ()) . unwrap_or (true) } # [doc = " Returns a `quic::Suite` for the ciphersuite, if supported."] pub fn quic_suite (& 'static self) -> Option < crate :: quic :: Suite > { self . quic . map (| quic | crate :: quic :: Suite { quic , suite : self }) } }
};
}
