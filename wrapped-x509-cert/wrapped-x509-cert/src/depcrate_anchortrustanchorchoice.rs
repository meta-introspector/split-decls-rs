// Generated macro for TrustAnchorChoice (enum)
macro_rules! Depcrate_anchorTrustAnchorChoice {
() => {
// Module: crate::anchor
// Provides: {"TrustAnchorChoice"}
// Dependencies: {}
# [doc = " TrustAnchorInfo allows for the representation of a single trust anchor."] # [doc = " Defined in [RFC 5914 Section 3]."] # [doc = ""] # [doc = " ```text"] # [doc = " TrustAnchorChoice ::= CHOICE {"] # [doc = "   certificate  Certificate,"] # [doc = "   tbsCert      [1] EXPLICIT TBSCertificate,"] # [doc = "   taInfo       [2] EXPLICIT TrustAnchorInfo"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5914 Section 3]: https://www.rfc-editor.org/rfc/rfc5914#section-3"] # [derive (Clone , Debug , PartialEq , Eq , Choice)] # [allow (clippy :: large_enum_variant)] # [allow (missing_docs)] pub enum TrustAnchorChoice < P : Profile = Rfc5280 > { Certificate (CertificateInner < P >) , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , constructed = "true")] TbsCertificate (TbsCertificateInner < P >) , # [asn1 (context_specific = "2" , tag_mode = "EXPLICIT" , constructed = "true")] TaInfo (TrustAnchorInfo < P >) , }
};
}
