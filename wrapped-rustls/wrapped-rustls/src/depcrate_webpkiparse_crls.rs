// Generated macro for parse_crls (function)
macro_rules! Depcrate_webpkiparse_crls {
() => {
// Module: crate::webpki
// Provides: {"parse_crls"}
// Dependencies: {}
fn parse_crls (crls : Vec < CertificateRevocationListDer < '_ > > ,) -> Result < Vec < CertRevocationList < '_ > > , CertRevocationListError > { crls . iter () . map (| der | OwnedCertRevocationList :: from_der (der . as_ref ()) . map (Into :: into)) . collect :: < Result < Vec < _ > , _ > > () . map_err (crl_error) }
};
}
