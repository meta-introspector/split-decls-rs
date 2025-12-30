// Generated macro for impl_136 (impl)
macro_rules! Depcrate_request_builderimpl_136 {
() => {
// Module: crate::request::builder
// Provides: {"impl_136"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Asn1 (err) => write ! (f , "ASN.1 error: {err}") , Error :: PublicKey (err) => write ! (f , "public key error: {err}") , Error :: Signature (err) => write ! (f , "signature error: {err}") , Error :: NonUniqueRdn => write ! (f , "Each RelativeDistinguishedName MUST contain exactly one AttributeTypeAndValue.") , Error :: NonUniqueATV => write ! (f , "Each Name MUST NOT contain more than one instance of a given AttributeTypeAndValue") , Error :: InvalidAttribute { oid } => write ! (f , "Non-ordered attribute or invalid attribute found (oid={oid})") , Error :: MissingAttributes => write ! (f , "Not all required elements were specified") , } } }
};
}
