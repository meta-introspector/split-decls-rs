// Generated macro for CertReq (struct)
macro_rules! Depcrate_requestCertReq {
() => {
// Module: crate::request
// Provides: {"CertReq"}
// Dependencies: {}
# [doc = " PKCS#10 `CertificationRequest` as defined in [RFC 2986 Section 4]."] # [doc = ""] # [doc = " ```text"] # [doc = " CertificationRequest ::= SEQUENCE {"] # [doc = "     certificationRequestInfo CertificationRequestInfo,"] # [doc = "     signatureAlgorithm AlgorithmIdentifier{{ SignatureAlgorithms }},"] # [doc = "     signature          BIT STRING"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 2986 Section 4]: https://datatracker.ietf.org/doc/html/rfc2986#section-4"] # [derive (Clone , Debug , PartialEq , Eq , Sequence)] pub struct CertReq { # [doc = " Certification request information."] pub info : CertReqInfo , # [doc = " Signature algorithm identifier."] pub algorithm : AlgorithmIdentifier , # [doc = " Signature."] pub signature : BitString , }
};
}
