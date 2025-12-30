// Generated macro for AsResponseBytes (trait)
macro_rules! Depcrate_responseAsResponseBytes {
() => {
// Module: crate::response
// Provides: {"AsResponseBytes"}
// Dependencies: {}
# [doc = " Trait for encoding [`ResponseBytes`]"] pub trait AsResponseBytes : AssociatedOid + der :: Encode { # [doc = " Encodes the response bytes of successful OCSP responses"] fn to_response_bytes (& self) -> Result < ResponseBytes , der :: Error > { Ok (ResponseBytes { response_type : < Self as AssociatedOid > :: OID , response : OctetString :: new (self . to_der () ?) ? , }) } }
};
}
