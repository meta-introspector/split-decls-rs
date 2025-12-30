// Generated macro for macro_1911 (macro)
macro_rules! Depcrate_enumsmacro_1911 {
() => {
// Module: crate::enums
// Provides: {"macro_1911"}
// Dependencies: {}
enum_builder ! { # [doc = " The `SignatureScheme` TLS protocol enum.  Values in this enum are taken"] # [doc = " from the various RFCs covering TLS, and are listed by IANA."] # [doc = " The `Unknown` item is used when processing unrecognized ordinals."] # [repr (u16)] pub enum SignatureScheme { RSA_PKCS1_SHA1 => 0x0201 , ECDSA_SHA1_Legacy => 0x0203 , RSA_PKCS1_SHA256 => 0x0401 , ECDSA_NISTP256_SHA256 => 0x0403 , RSA_PKCS1_SHA384 => 0x0501 , ECDSA_NISTP384_SHA384 => 0x0503 , RSA_PKCS1_SHA512 => 0x0601 , ECDSA_NISTP521_SHA512 => 0x0603 , RSA_PSS_SHA256 => 0x0804 , RSA_PSS_SHA384 => 0x0805 , RSA_PSS_SHA512 => 0x0806 , ED25519 => 0x0807 , ED448 => 0x0808 , ML_DSA_44 => 0x0904 , ML_DSA_65 => 0x0905 , ML_DSA_87 => 0x0906 , } }
};
}
