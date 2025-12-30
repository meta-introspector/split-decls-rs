// Generated macro for impl_129 (impl)
macro_rules! Depcrate_builderimpl_129 {
() => {
// Module: crate::builder
// Provides: {"impl_129"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Asn1 (err) => write ! (f , "ASN.1 error: {err}") , Error :: PublicKey (err) => write ! (f , "public key error: {err}") , Error :: Signature (err) => write ! (f , "signature error: {err}") , } } }
};
}
