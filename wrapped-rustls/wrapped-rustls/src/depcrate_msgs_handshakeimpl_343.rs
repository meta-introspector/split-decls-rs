// Generated macro for impl_343 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_343 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_343"}
// Dependencies: {}
impl Codec < '_ > for ServerDhParams { fn encode (& self , bytes : & mut Vec < u8 >) { self . dh_p . encode (bytes) ; self . dh_g . encode (bytes) ; self . dh_ys . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { dh_p : PayloadU16 :: read (r) ? , dh_g : PayloadU16 :: read (r) ? , dh_ys : PayloadU16 :: read (r) ? , }) } }
};
}
