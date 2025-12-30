// Generated macro for impl_15 (impl)
macro_rules! Depcrate_local_storageimpl_15 {
() => {
// Module: crate::local_storage
// Provides: {"impl_15"}
// Dependencies: {}
impl Storage for LocalStorage { fn raw () -> web_sys :: Storage { web_sys :: window () . expect_throw ("no window") . local_storage () . expect_throw ("failed to get local_storage") . expect_throw ("no local storage") } }
};
}
