// Generated macro for macro_169 (macro)
macro_rules! Depcrate_msgs_enumsmacro_169 {
() => {
// Module: crate::msgs::enums
// Provides: {"macro_169"}
// Dependencies: {}
enum_builder ! { # [doc = " The Key Derivation Function (`Kdf`) type for HPKE operations."] # [doc = " Listed by IANA, as specified in [RFC 9180 Section 7.2]"] # [doc = ""] # [doc = " [RFC 9180 Section 7.2]: <https://datatracker.ietf.org/doc/html/rfc9180#name-key-derivation-functions-kd>"] # [repr (u16)] # [derive (Default)] pub enum HpkeKdf { # [default] HKDF_SHA256 => 0x0001 , HKDF_SHA384 => 0x0002 , HKDF_SHA512 => 0x0003 , } }
};
}
