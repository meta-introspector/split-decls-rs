// Generated macro for impl_281 (impl)
macro_rules! Depcrate_crl_typesimpl_281 {
() => {
// Module: crate::crl::types
// Provides: {"impl_281"}
// Dependencies: {}
impl < 'a > FromDer < 'a > for BorrowedRevokedCert < 'a > { fn from_der (reader : & mut untrusted :: Reader < 'a >) -> Result < Self , Error > { der :: nested (reader , Tag :: Sequence , Error :: TrailingData (DerTypeId :: RevokedCertEntry) , | der | { let serial_number = lenient_certificate_serial_number (der) . map_err (| _ | Error :: InvalidSerialNumber) ? . as_slice_less_safe () ; let revocation_date = UnixTime :: from_der (der) ? ; let mut revoked_cert = BorrowedRevokedCert { serial_number , revocation_date , reason_code : None , invalidity_date : None , } ; if der . at_end () { return Ok (revoked_cert) ; } let ext_seq = der :: expect_tag (der , Tag :: Sequence) ? ; if ext_seq . is_empty () { return Ok (revoked_cert) ; } let mut reader = untrusted :: Reader :: new (ext_seq) ; loop { der :: nested (& mut reader , Tag :: Sequence , Error :: TrailingData (DerTypeId :: RevokedCertificateExtension) , | ext_der | { revoked_cert . remember_extension (& Extension :: from_der (ext_der) ?) } ,) ? ; if reader . at_end () { break ; } } Ok (revoked_cert) } ,) } const TYPE_ID : DerTypeId = DerTypeId :: RevokedCertificate ; }
};
}
