// Generated macro for test (module)
macro_rules! Depcrate_os_macos_identitytest {
() => {
// Module: crate::os::macos::identity
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use tempfile :: tempdir ; use super :: * ; use crate :: os :: macos :: certificate :: SecCertificateExt ; use crate :: os :: macos :: import_export :: ImportOptions ; use crate :: os :: macos :: keychain :: CreateOptions ; use crate :: os :: macos :: test :: identity ; use crate :: test ; # [test] fn certificate () { let dir = p ! (tempdir ()) ; let identity = identity (dir . path ()) ; let certificate = p ! (identity . certificate ()) ; assert_eq ! ("foobar.com" , p ! (certificate . common_name ())) ; } # [test] fn private_key () { let dir = p ! (tempdir ()) ; let identity = identity (dir . path ()) ; p ! (identity . private_key ()) ; } # [test] fn with_certificate () { let dir = p ! (tempdir ()) ; let keychain = p ! (CreateOptions :: new () . password ("foobar") . create (dir . path () . join ("test.keychain"))) ; let key = include_bytes ! ("../../../test/server.key") ; p ! (ImportOptions :: new () . filename ("server.key") . keychain (& keychain) . import (key)) ; let cert = test :: certificate () ; p ! (SecIdentity :: with_certificate (& [keychain] , & cert)) ; } }
};
}
