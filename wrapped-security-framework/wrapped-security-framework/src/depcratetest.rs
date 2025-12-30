// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: certificate :: SecCertificate ; pub fn certificate () -> SecCertificate { let certificate = include_bytes ! ("../test/server.der") ; p ! (SecCertificate :: from_der (certificate)) } }
};
}
