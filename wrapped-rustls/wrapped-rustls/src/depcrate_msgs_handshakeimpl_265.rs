// Generated macro for impl_265 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_265 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_265"}
// Dependencies: {}
impl Codec < '_ > for CertificateStatusRequest { fn encode (& self , bytes : & mut Vec < u8 >) { match self { Self :: Ocsp (r) => r . encode (bytes) , Self :: Unknown ((typ , payload)) => { typ . encode (bytes) ; payload . encode (bytes) ; } } } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let typ = CertificateStatusType :: read (r) ? ; match typ { CertificateStatusType :: OCSP => { let ocsp_req = OcspCertificateStatusRequest :: read (r) ? ; Ok (Self :: Ocsp (ocsp_req)) } _ => { let data = Payload :: read (r) . into_owned () ; Ok (Self :: Unknown ((typ , data))) } } } }
};
}
