// Generated macro for impl_371 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_371 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_371"}
// Dependencies: {}
impl Codec < '_ > for NewSessionTicketPayloadTls13 { fn encode (& self , bytes : & mut Vec < u8 >) { (self . lifetime . as_secs () as u32) . encode (bytes) ; self . age_add . encode (bytes) ; self . nonce . encode (bytes) ; self . ticket . encode (bytes) ; self . extensions . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let lifetime = Duration :: from_secs (u32 :: read (r) ? as u64) ; let age_add = u32 :: read (r) ? ; let nonce = PayloadU8 :: read (r) ? ; let ticket = Arc :: new (match PayloadU16 :: < NonEmpty > :: read (r) { Err (InvalidMessage :: IllegalEmptyValue) => Err (InvalidMessage :: EmptyTicketValue) , Err (err) => Err (err) , Ok (pl) => Ok (PayloadU16 :: new (pl . 0)) , } ?) ; let extensions = NewSessionTicketExtensions :: read (r) ? ; Ok (Self { lifetime , age_add , nonce , ticket , extensions , }) } }
};
}
