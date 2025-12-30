// Generated macro for macro_168 (macro)
macro_rules! Depcrate_msgs_enumsmacro_168 {
() => {
// Module: crate::msgs::enums
// Provides: {"macro_168"}
// Dependencies: {}
enum_builder ! { # [doc = " The Key Encapsulation Mechanism (`Kem`) type for HPKE operations."] # [doc = " Listed by IANA, as specified in [RFC 9180 Section 7.1]"] # [doc = ""] # [doc = " [RFC 9180 Section 7.1]: <https://datatracker.ietf.org/doc/html/rfc9180#kemid-values>"] # [repr (u16)] pub enum HpkeKem { DHKEM_P256_HKDF_SHA256 => 0x0010 , DHKEM_P384_HKDF_SHA384 => 0x0011 , DHKEM_P521_HKDF_SHA512 => 0x0012 , DHKEM_X25519_HKDF_SHA256 => 0x0020 , DHKEM_X448_HKDF_SHA512 => 0x0021 , } }
};
}
