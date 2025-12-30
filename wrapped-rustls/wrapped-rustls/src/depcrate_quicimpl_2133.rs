// Generated macro for impl_2133 (impl)
macro_rules! Depcrate_quicimpl_2133 {
() => {
// Module: crate::quic
// Provides: {"impl_2133"}
// Dependencies: {}
impl < 'a > KeyBuilder < 'a > { pub (crate) fn new (secret : & OkmBlock , version : Version , alg : & 'a dyn Algorithm , hkdf : & 'a dyn Hkdf ,) -> Self { Self { expander : hkdf . expander_for_okm (secret) , version , alg , } } # [doc = " Derive packet keys"] pub (crate) fn packet_key (& self) -> Box < dyn PacketKey > { let aead_key_len = self . alg . aead_key_len () ; let packet_key = hkdf_expand_label_aead_key (self . expander . as_ref () , aead_key_len , self . version . packet_key_label () , & [] ,) ; let packet_iv = hkdf_expand_label (self . expander . as_ref () , self . version . packet_iv_label () , & []) ; self . alg . packet_key (packet_key , packet_iv) } # [doc = " Derive header protection keys"] pub (crate) fn header_protection_key (& self) -> Box < dyn HeaderProtectionKey > { let header_key = hkdf_expand_label_aead_key (self . expander . as_ref () , self . alg . aead_key_len () , self . version . header_key_label () , & [] ,) ; self . alg . header_protection_key (header_key) } }
};
}
