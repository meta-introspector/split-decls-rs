// Generated macro for test (module)
macro_rules! Depcrate_cert_storetest {
() => {
// Module: crate::cert_store
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: ctl_context :: CtlContext ; use super :: * ; # [test] fn load () { let cert = include_bytes ! ("../test/cert.der") ; let mut store = Memory :: new () . unwrap () ; store . add_encoded_certificate (cert) . unwrap () ; } # [test] fn create_ctl () { let cert = include_bytes ! ("../test/self-signed.badssl.com.cer") ; let mut store = Memory :: new () . unwrap () ; let cert = store . add_encoded_certificate (cert) . unwrap () ; CtlContext :: builder () . certificate (cert) . usage ("1.3.6.1.4.1.311.2.2.2") . encode_and_sign () . unwrap () ; } # [test] fn pfx_import () { let pfx = include_bytes ! ("../test/identity.p12") ; let store = PfxImportOptions :: new () . include_extended_properties (true) . password ("mypass") . import (pfx) . unwrap () ; assert_eq ! (store . certs () . count () , 2) ; let pkeys = store . certs () . filter (| c | { c . private_key () . compare_key (true) . silent (true) . acquire () . is_ok () }) . count () ; assert_eq ! (pkeys , 1) ; } }
};
}
