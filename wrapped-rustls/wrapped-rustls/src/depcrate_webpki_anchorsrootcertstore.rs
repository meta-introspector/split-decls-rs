// Generated macro for RootCertStore (struct)
macro_rules! Depcrate_webpki_anchorsRootCertStore {
() => {
// Module: crate::webpki::anchors
// Provides: {"RootCertStore"}
// Dependencies: {}
# [doc = " A container for root certificates able to provide a root-of-trust"] # [doc = " for connection authentication."] # [expect (clippy :: exhaustive_structs)] # [derive (Clone)] pub struct RootCertStore { # [doc = " The list of roots."] pub roots : Vec < TrustAnchor < 'static > > , }
};
}
