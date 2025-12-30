// Generated macro for impl_20 (impl)
macro_rules! Depcrate_session_storageimpl_20 {
() => {
// Module: crate::session_storage
// Provides: {"impl_20"}
// Dependencies: {}
impl Storage for SessionStorage { fn raw () -> web_sys :: Storage { web_sys :: window () . expect_throw ("no window") . session_storage () . expect_throw ("failed to get session_storage") . expect_throw ("no session storage") } }
};
}
