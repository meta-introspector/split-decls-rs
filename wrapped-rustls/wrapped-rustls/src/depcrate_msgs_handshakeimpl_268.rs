// Generated macro for impl_268 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_268 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_268"}
// Dependencies: {}
impl Codec < '_ > for PskKeyExchangeModes { fn encode (& self , bytes : & mut Vec < u8 >) { let inner = LengthPrefixedBuffer :: new (PskKeyExchangeMode :: SIZE_LEN , bytes) ; if self . psk_dhe { PskKeyExchangeMode :: PSK_DHE_KE . encode (inner . buf) ; } if self . psk { PskKeyExchangeMode :: PSK_KE . encode (inner . buf) ; } } fn read (reader : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let mut psk_dhe = false ; let mut psk = false ; for ke in TlsListIter :: < PskKeyExchangeMode > :: new (reader) ? { match ke ? { PskKeyExchangeMode :: PSK_DHE_KE => psk_dhe = true , PskKeyExchangeMode :: PSK_KE => psk = true , _ => continue , } ; } Ok (Self { psk_dhe , psk }) } }
};
}
