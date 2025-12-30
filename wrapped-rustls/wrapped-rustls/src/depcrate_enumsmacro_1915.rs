// Generated macro for macro_1915 (macro)
macro_rules! Depcrate_enumsmacro_1915 {
() => {
// Module: crate::enums
// Provides: {"macro_1915"}
// Dependencies: {}
enum_builder ! { # [doc = " The `SignatureAlgorithm` TLS protocol enum.  Values in this enum are taken"] # [doc = " from the various RFCs covering TLS, and are listed by IANA."] # [doc = " The `Unknown` item is used when processing unrecognized ordinals."] # [repr (u8)] pub enum SignatureAlgorithm { Anonymous => 0x00 , RSA => 0x01 , DSA => 0x02 , ECDSA => 0x03 , ED25519 => 0x07 , ED448 => 0x08 , } }
};
}
