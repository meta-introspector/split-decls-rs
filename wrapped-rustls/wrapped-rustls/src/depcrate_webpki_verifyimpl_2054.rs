// Generated macro for impl_2054 (impl)
macro_rules! Depcrate_webpki_verifyimpl_2054 {
() => {
// Module: crate::webpki::verify
// Provides: {"impl_2054"}
// Dependencies: {}
impl WebPkiSupportedAlgorithms { # [doc = " Return all the `scheme` items in `mapping`, maintaining order."] pub fn supported_schemes (& self) -> Vec < SignatureScheme > { self . mapping . iter () . map (| item | item . 0) . collect () } # [doc = " Return the first item in `mapping` that matches `scheme`."] fn convert_scheme (& self , scheme : SignatureScheme ,) -> Result < & [& 'static dyn SignatureVerificationAlgorithm] , Error > { self . mapping . iter () . filter_map (| item | if item . 0 == scheme { Some (item . 1) } else { None }) . next () . ok_or_else (| | PeerMisbehaved :: SignedHandshakeWithUnadvertisedSigScheme . into ()) } # [doc = " Return `true` if all cryptography is FIPS-approved."] pub fn fips (& self) -> bool { self . all . iter () . all (| alg | alg . fips ()) && self . mapping . iter () . all (| item | item . 1 . iter () . all (| alg | alg . fips ())) } }
};
}
