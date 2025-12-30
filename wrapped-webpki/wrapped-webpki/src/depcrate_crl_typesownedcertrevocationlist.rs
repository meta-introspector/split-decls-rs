// Generated macro for OwnedCertRevocationList (struct)
macro_rules! Depcrate_crl_typesOwnedCertRevocationList {
() => {
// Module: crate::crl::types
// Provides: {"OwnedCertRevocationList"}
// Dependencies: {}
# [doc = " Owned representation of a RFC 5280[^1] profile Certificate Revocation List (CRL)."] # [doc = ""] # [doc = " [^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>"] # [cfg (feature = "alloc")] # [derive (Debug , Clone , Hash)] pub struct OwnedCertRevocationList { # [doc = " A map of the revoked certificates contained in then CRL, keyed by the DER encoding"] # [doc = " of the revoked cert's serial number."] revoked_certs : BTreeMap < Vec < u8 > , OwnedRevokedCert > , issuer : Vec < u8 > , issuing_distribution_point : Option < Vec < u8 > > , signed_data : OwnedSignedData , next_update : UnixTime , }
};
}
