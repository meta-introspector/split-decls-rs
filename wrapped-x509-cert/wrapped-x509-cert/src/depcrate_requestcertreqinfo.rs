// Generated macro for CertReqInfo (struct)
macro_rules! Depcrate_requestCertReqInfo {
() => {
// Module: crate::request
// Provides: {"CertReqInfo"}
// Dependencies: {}
# [doc = " PKCS#10 `CertificationRequestInfo` as defined in [RFC 2986 Section 4]."] # [doc = ""] # [doc = " ```text"] # [doc = " CertificationRequestInfo ::= SEQUENCE {"] # [doc = "     version       INTEGER { v1(0) } (v1,...),"] # [doc = "     subject       Name,"] # [doc = "     subjectPKInfo SubjectPublicKeyInfo{{ PKInfoAlgorithms }},"] # [doc = "     attributes    [0] Attributes{{ CRIAttributes }}"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 2986 Section 4]: https://datatracker.ietf.org/doc/html/rfc2986#section-4"] # [derive (Clone , Debug , PartialEq , Eq , Sequence)] pub struct CertReqInfo { # [doc = " Certification request version."] pub version : Version , # [doc = " Subject name."] pub subject : Name , # [doc = " Subject public key info."] pub public_key : SubjectPublicKeyInfo , # [doc = " Request attributes."] # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT")] pub attributes : Attributes , }
};
}
