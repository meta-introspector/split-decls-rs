// Generated macro for WebPkiSupportedAlgorithms (struct)
macro_rules! Depcrate_webpki_verifyWebPkiSupportedAlgorithms {
() => {
// Module: crate::webpki::verify
// Provides: {"WebPkiSupportedAlgorithms"}
// Dependencies: {}
# [doc = " Describes which `webpki` signature verification algorithms are supported and"] # [doc = " how they map to TLS [`SignatureScheme`]s."] # [expect (clippy :: exhaustive_structs)] # [derive (Clone , Copy)] pub struct WebPkiSupportedAlgorithms { # [doc = " A list of all supported signature verification algorithms."] # [doc = ""] # [doc = " Used for verifying certificate chains."] # [doc = ""] # [doc = " The order of this list is not significant."] pub all : & 'static [& 'static dyn SignatureVerificationAlgorithm] , # [doc = " A mapping from TLS `SignatureScheme`s to matching webpki signature verification algorithms."] # [doc = ""] # [doc = " This is one (`SignatureScheme`) to many ([`SignatureVerificationAlgorithm`]) because"] # [doc = " (depending on the protocol version) there is not necessary a 1-to-1 mapping."] # [doc = ""] # [doc = " For TLS1.2, all `SignatureVerificationAlgorithm`s are tried in sequence."] # [doc = ""] # [doc = " For TLS1.3, only the first is tried."] # [doc = ""] # [doc = " The supported schemes in this mapping is communicated to the peer and the order is significant."] # [doc = " The first mapping is our highest preference."] pub mapping : & 'static [(SignatureScheme , & 'static [& 'static dyn SignatureVerificationAlgorithm] ,)] , }
};
}
