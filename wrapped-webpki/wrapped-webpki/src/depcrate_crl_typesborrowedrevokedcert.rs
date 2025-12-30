// Generated macro for BorrowedRevokedCert (struct)
macro_rules! Depcrate_crl_typesBorrowedRevokedCert {
() => {
// Module: crate::crl::types
// Provides: {"BorrowedRevokedCert"}
// Dependencies: {}
# [doc = " Borrowed representation of a RFC 5280[^1] profile Certificate Revocation List (CRL) revoked"] # [doc = " certificate entry."] # [doc = ""] # [doc = " [^1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5>"] # [derive (Debug)] pub struct BorrowedRevokedCert < 'a > { # [doc = " Serial number of the revoked certificate."] pub serial_number : & 'a [u8] , # [doc = " The date at which the CA processed the revocation."] pub revocation_date : UnixTime , # [doc = " Identifies the reason for the certificate revocation. When absent, the revocation reason"] # [doc = " is assumed to be RevocationReason::Unspecified. For consistency with other extensions"] # [doc = " and to ensure only one revocation reason extension may be present we maintain this field"] # [doc = " as optional instead of defaulting to unspecified."] pub reason_code : Option < RevocationReason > , # [doc = " Provides the date on which it is known or suspected that the private key was compromised or"] # [doc = " that the certificate otherwise became invalid. This date may be earlier than the revocation"] # [doc = " date which is the date at which the CA processed the revocation."] pub invalidity_date : Option < UnixTime > , }
};
}
