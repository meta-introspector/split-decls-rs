// Generated macro for impl_15 (impl)
macro_rules! Depcrate_basicimpl_15 {
() => {
// Module: crate::basic
// Provides: {"impl_15"}
// Dependencies: {}
impl ResponseData { # [doc = " Returns the response's nonce value, if any. This method will return `None` if the response"] # [doc = " has no `Nonce` extension or decoding of the `Nonce` extension fails."] pub fn nonce (& self) -> Option < Nonce > { match & self . response_extensions { Some (extns) => { let mut filter = extns . iter () . filter (| e | e . extn_id == ID_PKIX_OCSP_NONCE) ; match filter . next () { Some (extn) => Nonce :: from_der (extn . extn_value . as_bytes ()) . ok () , None => None , } } None => None , } } }
};
}
