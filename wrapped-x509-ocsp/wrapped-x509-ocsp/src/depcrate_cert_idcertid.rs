// Generated macro for CertId (struct)
macro_rules! Depcrate_cert_idCertId {
() => {
// Module: crate::cert_id
// Provides: {"CertId"}
// Dependencies: {}
# [doc = " CertID structure as defined in [RFC 6960 Section 4.1.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " CertID ::= SEQUENCE {"] # [doc = "    hashAlgorithm           AlgorithmIdentifier,"] # [doc = "    issuerNameHash          OCTET STRING, -- Hash of issuer's DN"] # [doc = "    issuerKeyHash           OCTET STRING, -- Hash of issuer's public key"] # [doc = "    serialNumber            CertificateSerialNumber }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.1.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CertId < P : Profile + 'static = Rfc5280 > { pub hash_algorithm : AlgorithmIdentifierOwned , pub issuer_name_hash : OctetString , pub issuer_key_hash : OctetString , pub serial_number : SerialNumber < P > , }
};
}
