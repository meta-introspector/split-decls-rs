// Generated macro for macro_170 (macro)
macro_rules! Depcrate_msgs_enumsmacro_170 {
() => {
// Module: crate::msgs::enums
// Provides: {"macro_170"}
// Dependencies: {}
enum_builder ! { # [doc = " The Authenticated Encryption with Associated Data (`Aead`) type for HPKE operations."] # [doc = " Listed by IANA, as specified in [RFC 9180 Section 7.3]"] # [doc = ""] # [doc = " [RFC 9180 Section 7.3]: <https://datatracker.ietf.org/doc/html/rfc9180#name-authenticated-encryption-wi>"] # [repr (u16)] # [derive (Default)] pub enum HpkeAead { # [default] AES_128_GCM => 0x0001 , AES_256_GCM => 0x0002 , CHACHA20_POLY_1305 => 0x0003 , EXPORT_ONLY => 0xFFFF , } }
};
}
