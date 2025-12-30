// Generated macro for impl_2120 (impl)
macro_rules! Depcrate_quicimpl_2120 {
() => {
// Module: crate::quic
// Provides: {"impl_2120"}
// Dependencies: {}
impl Secrets { pub (crate) fn new (client : OkmBlock , server : OkmBlock , suite : & 'static Tls13CipherSuite , quic : & 'static dyn Algorithm , side : Side , version : Version ,) -> Self { Self { client , server , suite , quic , side , version , } } # [doc = " Derive the next set of packet keys"] pub fn next_packet_keys (& mut self) -> PacketKeySet { let keys = PacketKeySet :: new (self) ; self . update () ; keys } pub (crate) fn update (& mut self) { self . client = hkdf_expand_label_block (self . suite . hkdf_provider . expander_for_okm (& self . client) . as_ref () , self . version . key_update_label () , & [] ,) ; self . server = hkdf_expand_label_block (self . suite . hkdf_provider . expander_for_okm (& self . server) . as_ref () , self . version . key_update_label () , & [] ,) ; } fn local_remote (& self) -> (& OkmBlock , & OkmBlock) { match self . side { Side :: Client => (& self . client , & self . server) , Side :: Server => (& self . server , & self . client) , } } }
};
}
