// Generated macro for impl_314 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_314 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_314"}
// Dependencies: {}
impl < 'a > Codec < 'a > for CertificateChain < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { Vec :: encode (& self . 0 , bytes) } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { let mut ret = Vec :: new () ; for item in TlsListIter :: < CertificateDer < 'a > > :: new (r) ? { ret . push (item ?) ; } Ok (Self (ret)) } }
};
}
