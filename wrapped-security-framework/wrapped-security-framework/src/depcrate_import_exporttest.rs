// Generated macro for test (module)
macro_rules! Depcrate_import_exporttest {
() => {
// Module: crate::import_export
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn missing_passphrase () { let data = include_bytes ! ("../test/server.p12") ; assert ! (Pkcs12ImportOptions :: new () . import (data) . is_err ()) ; } }
};
}
