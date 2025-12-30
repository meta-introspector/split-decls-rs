// Generated macro for TrustAnchorInfo (struct)
macro_rules! Depcrate_anchorTrustAnchorInfo {
() => {
// Module: crate::anchor
// Provides: {"TrustAnchorInfo"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " TrustAnchorInfo ::= SEQUENCE {"] # [doc = "     version         TrustAnchorInfoVersion DEFAULT v1,"] # [doc = "     pubKey          SubjectPublicKeyInfo,"] # [doc = "     keyId           KeyIdentifier,"] # [doc = "     taTitle         TrustAnchorTitle OPTIONAL,"] # [doc = "     certPath        CertPathControls OPTIONAL,"] # [doc = "     exts            [1] EXPLICIT Extensions   OPTIONAL,"] # [doc = "     taTitleLangTag  [2] UTF8String OPTIONAL"] # [doc = " }"] # [doc = ""] # [doc = " TrustAnchorInfoVersion ::= INTEGER { v1(1) }"] # [doc = ""] # [doc = " TrustAnchorTitle ::= UTF8String (SIZE (1..64))"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq , Sequence)] # [allow (missing_docs)] pub struct TrustAnchorInfo < P : Profile = Rfc5280 > { # [asn1 (default = "Default::default")] pub version : Version , pub pub_key : SubjectPublicKeyInfo , pub key_id : OctetString , # [asn1 (optional = "true")] pub ta_title : Option < String > , # [asn1 (optional = "true")] pub cert_path : Option < CertPathControls < P > > , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , optional = "true")] pub extensions : Option < Extensions > , # [asn1 (context_specific = "2" , tag_mode = "IMPLICIT" , optional = "true")] pub ta_title_lang_tag : Option < String > , }
};
}
