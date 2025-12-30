// Generated macro for impl_399 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_399 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_399"}
// Dependencies: {}
impl Codec < '_ > for EncryptedClientHello { fn encode (& self , bytes : & mut Vec < u8 >) { match self { Self :: Outer (payload) => { EchClientHelloType :: ClientHelloOuter . encode (bytes) ; payload . encode (bytes) ; } Self :: Inner => { EchClientHelloType :: ClientHelloInner . encode (bytes) ; } } } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { match EchClientHelloType :: read (r) ? { EchClientHelloType :: ClientHelloOuter => { Ok (Self :: Outer (EncryptedClientHelloOuter :: read (r) ?)) } EchClientHelloType :: ClientHelloInner => Ok (Self :: Inner) , _ => Err (InvalidMessage :: InvalidContentType) , } } }
};
}
