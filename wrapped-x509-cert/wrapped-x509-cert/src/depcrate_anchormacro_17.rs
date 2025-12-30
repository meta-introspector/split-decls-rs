// Generated macro for macro_17 (macro)
macro_rules! Depcrate_anchormacro_17 {
() => {
// Module: crate::anchor
// Provides: {"macro_17"}
// Dependencies: {}
flags ! { # [doc = " Certificate policies as defined in [RFC 5280 Section 4.2.1.13]."] # [doc = ""] # [doc = " ```text"] # [doc = " CertPolicyFlags ::= BIT STRING {"] # [doc = "     inhibitPolicyMapping    (0),"] # [doc = "     requireExplicitPolicy   (1),"] # [doc = "     inhibitAnyPolicy        (2)"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 4.2.1.13]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.13"] # [allow (missing_docs)] pub enum CertPolicies : u8 { InhibitPolicyMapping = 1 << 0 , RequireExplicitPolicy = 1 << 1 , InhibitAnyPolicy = 1 << 2 , } }
};
}
