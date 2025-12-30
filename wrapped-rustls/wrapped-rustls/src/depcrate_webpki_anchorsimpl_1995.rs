// Generated macro for impl_1995 (impl)
macro_rules! Depcrate_webpki_anchorsimpl_1995 {
() => {
// Module: crate::webpki::anchors
// Provides: {"impl_1995"}
// Dependencies: {}
impl FromIterator < TrustAnchor < 'static > > for RootCertStore { fn from_iter < T : IntoIterator < Item = TrustAnchor < 'static > > > (iter : T) -> Self { Self { roots : iter . into_iter () . collect () , } } }
};
}
