// Generated macro for BorrowedCertRevocationList (struct)
macro_rules! Depcrate_crl_typesBorrowedCertRevocationList {
() => {
// Module: crate::crl::types
// Provides: {"BorrowedCertRevocationList"}
// Dependencies: {}
# [doc = " Borrowed representation of a RFC 5280[^1] profile Certificate Revocation List (CRL)."] # [doc = ""] # [doc = " [^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>"] # [derive (Debug)] pub struct BorrowedCertRevocationList < 'a > { # [doc = " A `SignedData` structure that can be passed to `verify_signed_data`."] signed_data : SignedData < 'a > , # [doc = " Identifies the entity that has signed and issued this"] # [doc = " CRL."] issuer : untrusted :: Input < 'a > , # [doc = " An optional CRL extension that identifies the CRL distribution point and scope for the CRL."] issuing_distribution_point : Option < untrusted :: Input < 'a > > , # [doc = " List of certificates revoked by the issuer in this CRL."] revoked_certs : untrusted :: Input < 'a > , next_update : UnixTime , }
};
}
