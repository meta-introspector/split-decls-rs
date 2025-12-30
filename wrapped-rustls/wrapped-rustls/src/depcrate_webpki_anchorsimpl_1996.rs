// Generated macro for impl_1996 (impl)
macro_rules! Depcrate_webpki_anchorsimpl_1996 {
() => {
// Module: crate::webpki::anchors
// Provides: {"impl_1996"}
// Dependencies: {}
impl Extend < TrustAnchor < 'static > > for RootCertStore { fn extend < T : IntoIterator < Item = TrustAnchor < 'static > > > (& mut self , iter : T) { self . roots . extend (iter) ; } }
};
}
