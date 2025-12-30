// Generated macro for local_root_store (function)
macro_rules! Depcrate_testlocal_root_store {
() => {
// Module: crate::test
// Provides: {"local_root_store"}
// Dependencies: {}
fn local_root_store () -> CertStore { if env :: var ("APPVEYOR") . is_ok () || env :: var ("CI") . is_ok () { CertStore :: open_local_machine ("Root") . unwrap () } else { CertStore :: open_current_user ("Root") . unwrap () } }
};
}
