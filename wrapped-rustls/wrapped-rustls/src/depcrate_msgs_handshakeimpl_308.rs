// Generated macro for impl_308 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_308 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_308"}
// Dependencies: {}
impl Codec < '_ > for ServerHelloPayload { fn encode (& self , bytes : & mut Vec < u8 >) { self . payload_encode (bytes , Encoding :: Standard) } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let session_id = SessionId :: read (r) ? ; let suite = CipherSuite :: read (r) ? ; let compression = Compression :: read (r) ? ; let extensions = Box :: new (if r . any_left () { ServerExtensions :: read (r) ? } else { ServerExtensions :: default () } . into_owned () ,) ; let ret = Self { legacy_version : ProtocolVersion :: Unknown (0) , random : ZERO_RANDOM , session_id , cipher_suite : suite , compression_method : compression , extensions , } ; r . expect_empty ("ServerHelloPayload") . map (| _ | ret) } }
};
}
