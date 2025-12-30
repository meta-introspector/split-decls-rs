// Generated macro for root_cert_store_debug (function)
macro_rules! Depcrate_webpki_anchorsroot_cert_store_debug {
() => {
// Module: crate::webpki::anchors
// Provides: {"root_cert_store_debug"}
// Dependencies: {}
# [test] fn root_cert_store_debug () { use core :: iter ; use pki_types :: Der ; let ta = TrustAnchor { subject : Der :: from_slice (& []) , subject_public_key_info : Der :: from_slice (& []) , name_constraints : None , } ; let store = RootCertStore :: from_iter (iter :: repeat_n (ta , 138)) ; assert_eq ! (format ! ("{store:?}") , "RootCertStore { roots: \"(138 roots)\" }") ; }
};
}
