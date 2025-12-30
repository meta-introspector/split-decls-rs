// Generated macro for impl_293 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_293 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_293"}
// Dependencies: {}
impl Codec < '_ > for ClientHelloPayload { fn encode (& self , bytes : & mut Vec < u8 >) { self . payload_encode (bytes , Encoding :: Standard) } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let ret = Self { client_version : ProtocolVersion :: read (r) ? , random : Random :: read (r) ? , session_id : SessionId :: read (r) ? , cipher_suites : Vec :: read (r) ? , compression_methods : Vec :: read (r) ? , extensions : Box :: new (ClientExtensions :: read (r) ? . into_owned ()) , } ; match r . any_left () { true => Err (InvalidMessage :: TrailingData ("ClientHelloPayload")) , false => Ok (ret) , } } }
};
}
