// Generated macro for impl_335 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_335 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_335"}
// Dependencies: {}
impl Codec < '_ > for ClientEcdhParams { fn encode (& self , bytes : & mut Vec < u8 >) { self . public . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let pb = PayloadU8 :: read (r) ? ; Ok (Self { public : pb }) } }
};
}
