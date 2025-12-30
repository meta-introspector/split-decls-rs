// Generated macro for impl_2140 (impl)
macro_rules! Depcrate_quicimpl_2140 {
() => {
// Module: crate::quic
// Provides: {"impl_2140"}
// Dependencies: {}
impl Version { fn initial_salt (self) -> & 'static [u8 ; 20] { match self { Self :: V1 => & [0x38 , 0x76 , 0x2c , 0xf7 , 0xf5 , 0x59 , 0x34 , 0xb3 , 0x4d , 0x17 , 0x9a , 0xe6 , 0xa4 , 0xc8 , 0x0c , 0xad , 0xcc , 0xbb , 0x7f , 0x0a ,] , Self :: V2 => & [0x0d , 0xed , 0xe3 , 0xde , 0xf7 , 0x00 , 0xa6 , 0xdb , 0x81 , 0x93 , 0x81 , 0xbe , 0x6e , 0x26 , 0x9d , 0xcb , 0xf9 , 0xbd , 0x2e , 0xd9 ,] , } } # [doc = " Key derivation label for packet keys."] pub (crate) fn packet_key_label (& self) -> & 'static [u8] { match self { Self :: V1 => b"quic key" , Self :: V2 => b"quicv2 key" , } } # [doc = " Key derivation label for packet \"IV\"s."] pub (crate) fn packet_iv_label (& self) -> & 'static [u8] { match self { Self :: V1 => b"quic iv" , Self :: V2 => b"quicv2 iv" , } } # [doc = " Key derivation for header keys."] pub (crate) fn header_key_label (& self) -> & 'static [u8] { match self { Self :: V1 => b"quic hp" , Self :: V2 => b"quicv2 hp" , } } fn key_update_label (& self) -> & 'static [u8] { match self { Self :: V1 => b"quic ku" , Self :: V2 => b"quicv2 ku" , } } }
};
}
