// Generated macro for impl_366 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_366 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_366"}
// Dependencies: {}
impl Codec < '_ > for NewSessionTicketPayload { fn encode (& self , bytes : & mut Vec < u8 >) { (self . lifetime_hint . as_secs () as u32) . encode (bytes) ; self . ticket . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { lifetime_hint : Duration :: from_secs (u32 :: read (r) ? as u64) , ticket : Arc :: new (PayloadU16 :: read (r) ?) , }) } }
};
}
