// Generated macro for impl_49 (impl)
macro_rules! Depcrate_requestimpl_49 {
() => {
// Module: crate::request
// Provides: {"impl_49"}
// Dependencies: {}
impl TbsRequest { # [doc = " Returns the request's nonce value, if any. This method will return `None` if the request"] # [doc = " has no `Nonce` extension or decoding of the `Nonce` extension fails."] pub fn nonce (& self) -> Option < Nonce > { match & self . request_extensions { Some (extns) => { let mut filter = extns . iter () . filter (| e | e . extn_id == ID_PKIX_OCSP_NONCE) ; match filter . next () { Some (extn) => Nonce :: from_der (extn . extn_value . as_bytes ()) . ok () , None => None , } } None => None , } } }
};
}
