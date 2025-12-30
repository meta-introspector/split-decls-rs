// Generated macro for test (module)
macro_rules! Depcrate_os_macos_certificatetest {
() => {
// Module: crate::os::macos::certificate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: test :: certificate ; use std :: collections :: HashMap ; # [test] fn common_name () { let certificate = certificate () ; assert_eq ! ("foobar.com" , p ! (certificate . common_name ())) ; } # [test] # [allow (deprecated)] fn public_key () { let certificate = certificate () ; p ! (certificate . public_key ()) ; } # [test] fn fingerprint () { let certificate = certificate () ; let fingerprint = p ! (certificate . fingerprint ()) ; assert_eq ! ("af9dd180a326ae08b37e6398f9262f8b9d4c55674a233a7c84975024f873655d" , hex :: encode (fingerprint)) ; } # [test] fn signature_algorithm () { let certificate = certificate () ; let properties = certificate . properties (Some (& [CertificateOid :: x509_v1_signature_algorithm ()])) . unwrap () ; let value = properties . get (CertificateOid :: x509_v1_signature_algorithm ()) . unwrap () ; let PropertyType :: Section (section) = value . get () else { panic ! () } ; let properties = section . iter () . map (| p | (p . label () . to_string () , p . get ())) . collect :: < HashMap < _ , _ > > () ; let algorithm = match properties ["Algorithm"] { PropertyType :: String (ref s) => s . to_string () , _ => panic ! () , } ; assert_eq ! (algorithm , "1.2.840.113549.1.1.5") ; } }
};
}
