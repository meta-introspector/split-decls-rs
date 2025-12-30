// Generated macro for impl_303 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_303 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_303"}
// Dependencies: {}
impl Codec < '_ > for HelloRetryRequest { fn encode (& self , bytes : & mut Vec < u8 >) { self . payload_encode (bytes , Encoding :: Standard) } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let session_id = SessionId :: read (r) ? ; let cipher_suite = CipherSuite :: read (r) ? ; let compression = Compression :: read (r) ? ; if compression != Compression :: Null { return Err (InvalidMessage :: UnsupportedCompression) ; } Ok (Self { legacy_version : ProtocolVersion :: Unknown (0) , session_id , cipher_suite , extensions : HelloRetryRequestExtensions :: read (r) ? . into_owned () , }) } }
};
}
