// Generated macro for impl_250 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_250 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_250"}
// Dependencies: {}
impl Codec < '_ > for KeyShareEntry { fn encode (& self , bytes : & mut Vec < u8 >) { self . group . encode (bytes) ; self . payload . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let group = NamedGroup :: read (r) ? ; let payload = PayloadU16 :: read (r) ? ; Ok (Self { group , payload }) } }
};
}
