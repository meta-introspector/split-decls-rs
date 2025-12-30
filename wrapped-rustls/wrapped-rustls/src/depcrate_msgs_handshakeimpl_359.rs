// Generated macro for impl_359 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_359 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_359"}
// Dependencies: {}
impl Codec < '_ > for CertificateRequestPayload { fn encode (& self , bytes : & mut Vec < u8 >) { self . certtypes . encode (bytes) ; self . sigschemes . encode (bytes) ; self . canames . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let certtypes = Vec :: read (r) ? ; let sigschemes = Vec :: read (r) ? ; let canames = Vec :: read (r) ? ; if sigschemes . is_empty () { warn ! ("meaningless CertificateRequest message") ; Err (InvalidMessage :: NoSignatureSchemes) } else { Ok (Self { certtypes , sigschemes , canames , }) } } }
};
}
