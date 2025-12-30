// Generated macro for SubjectPublicKeyInfo (struct)
macro_rules! Depcrate_spkiSubjectPublicKeyInfo {
() => {
// Module: crate::spki
// Provides: {"SubjectPublicKeyInfo"}
// Dependencies: {}
# [doc = " X.509 `SubjectPublicKeyInfo` (SPKI) as defined in [RFC 5280 § 4.1.2.7]."] # [doc = ""] # [doc = " ASN.1 structure containing an [`AlgorithmIdentifier`] and public key"] # [doc = " data in an algorithm specific format."] # [doc = ""] # [doc = " ```text"] # [doc = "    SubjectPublicKeyInfo  ::=  SEQUENCE  {"] # [doc = "         algorithm            AlgorithmIdentifier,"] # [doc = "         subjectPublicKey     BIT STRING  }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 § 4.1.2.7]: https://tools.ietf.org/html/rfc5280#section-4.1.2.7"] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [derive (Clone , Debug , Eq , PartialEq)] pub struct SubjectPublicKeyInfo < Params , Key > { # [doc = " X.509 [`AlgorithmIdentifier`] for the public key type"] pub algorithm : AlgorithmIdentifier < Params > , # [doc = " Public key data"] pub subject_public_key : Key , }
};
}
