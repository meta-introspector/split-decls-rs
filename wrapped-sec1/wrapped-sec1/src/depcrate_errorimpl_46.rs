// Generated macro for impl_46 (impl)
macro_rules! Depcrate_errorimpl_46 {
() => {
// Module: crate::error
// Provides: {"impl_46"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { # [cfg (feature = "der")] Error :: Asn1 (err) => write ! (f , "SEC1 ASN.1 error: {err}") , Error :: Crypto => f . write_str ("SEC1 cryptographic error") , Error :: PointEncoding => f . write_str ("elliptic curve point encoding error") , Error :: Version => f . write_str ("SEC1 version error") , } } }
};
}
