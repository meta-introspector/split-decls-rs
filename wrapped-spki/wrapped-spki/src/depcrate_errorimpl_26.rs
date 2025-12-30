// Generated macro for impl_26 (impl)
macro_rules! Depcrate_errorimpl_26 {
() => {
// Module: crate::error
// Provides: {"impl_26"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: AlgorithmParametersMissing => { f . write_str ("AlgorithmIdentifier parameters missing") } Error :: Asn1 (err) => write ! (f , "ASN.1 error: {err}") , Error :: KeyMalformed => f . write_str ("SPKI cryptographic key data malformed") , Error :: OidUnknown { oid } => { write ! (f , "unknown/unsupported algorithm OID: {oid}") } } } }
};
}
