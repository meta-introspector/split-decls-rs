// Generated macro for impl_253 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_253 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_253"}
// Dependencies: {}
impl Codec < '_ > for PresharedKeyIdentity { fn encode (& self , bytes : & mut Vec < u8 >) { self . identity . encode (bytes) ; self . obfuscated_ticket_age . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { identity : PayloadU16 :: read (r) ? , obfuscated_ticket_age : u32 :: read (r) ? , }) } }
};
}
