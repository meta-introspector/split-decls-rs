// Generated macro for macro_1917 (macro)
macro_rules! Depcrate_enumsmacro_1917 {
() => {
// Module: crate::enums
// Provides: {"macro_1917"}
// Dependencies: {}
enum_builder ! { # [doc = " The `CertificateType` enum sent in the cert_type extensions."] # [doc = " Values in this enum are taken from the various RFCs covering TLS, and are listed by IANA."] # [doc = ""] # [doc = " [RFC 6091 Section 5]: <https://datatracker.ietf.org/doc/html/rfc6091#section-5>"] # [doc = " [RFC 7250 Section 7]: <https://datatracker.ietf.org/doc/html/rfc7250#section-7>"] # [repr (u8)] # [derive (Default)] pub enum CertificateType { # [default] X509 => 0x00 , RawPublicKey => 0x02 , } }
};
}
