// Generated macro for impl_2069 (impl)
macro_rules! Depcrate_webpkiimpl_2069 {
() => {
// Module: crate::webpki
// Provides: {"impl_2069"}
// Dependencies: {}
impl fmt :: Display for VerifierBuilderError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: NoRootAnchors => write ! (f , "no root trust anchors were provided") , Self :: InvalidCrl (e) => write ! (f , "provided CRL could not be parsed: {e:?}") , } } }
};
}
