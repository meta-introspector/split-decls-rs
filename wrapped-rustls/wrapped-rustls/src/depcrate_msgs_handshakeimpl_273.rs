// Generated macro for impl_273 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_273 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_273"}
// Dependencies: {}
impl Codec < '_ > for SupportedProtocolVersions { fn encode (& self , bytes : & mut Vec < u8 >) { let inner = LengthPrefixedBuffer :: new (Self :: LIST_LENGTH , bytes) ; if self . tls13 { ProtocolVersion :: TLSv1_3 . encode (inner . buf) ; } if self . tls12 { ProtocolVersion :: TLSv1_2 . encode (inner . buf) ; } } fn read (reader : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let mut tls12 = false ; let mut tls13 = false ; for pv in TlsListIter :: < ProtocolVersion > :: new (reader) ? { match pv ? { ProtocolVersion :: TLSv1_3 => tls13 = true , ProtocolVersion :: TLSv1_2 => tls12 = true , _ => continue , } ; } Ok (Self { tls13 , tls12 }) } }
};
}
