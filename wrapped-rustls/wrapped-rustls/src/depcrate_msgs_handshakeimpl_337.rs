// Generated macro for impl_337 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_337 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_337"}
// Dependencies: {}
impl Codec < '_ > for ClientDhParams { fn encode (& self , bytes : & mut Vec < u8 >) { self . public . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { public : PayloadU16 :: read (r) ? , }) } }
};
}
