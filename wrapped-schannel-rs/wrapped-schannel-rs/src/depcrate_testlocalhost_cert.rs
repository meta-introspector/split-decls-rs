// Generated macro for localhost_cert (function)
macro_rules! Depcrate_testlocalhost_cert {
() => {
// Module: crate::test
// Provides: {"localhost_cert"}
// Dependencies: {}
fn localhost_cert () -> Option < CertContext > { if env :: var ("SCHANNEL_RS_SKIP_SERVER_TESTS") . is_ok () { return None ; } static INIT : Once = Once :: new () ; INIT . call_once (| | { for cert in local_root_store () . certs () { let name = match cert . friendly_name () { Ok (name) => name , Err (_) => continue , } ; if name != FRIENDLY_NAME { continue ; } if ! cert . is_time_valid () . unwrap () { io :: stdout () . write_all (br#"

The schannel-rs test suite is about to delete an old copy of one of its
certificates from your root trust store. This certificate was only valid for one
day and it is no longer needed. The host should be "localhost" and the
description should mention "schannel".

"# ,) . unwrap () ; cert . delete () . unwrap () ; } else { return ; } } install_certificate () . unwrap () ; }) ; for cert in local_root_store () . certs () { let name = match cert . friendly_name () { Ok (name) => name , Err (_) => continue , } ; if name == FRIENDLY_NAME { return Some (cert) ; } } panic ! ("couldn't find a cert") ; }
};
}
