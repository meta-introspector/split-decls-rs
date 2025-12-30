// Generated macro for test (module)
macro_rules! Depcrate_thread_atomics_oneshottest {
() => {
// Module: crate::thread::atomics::oneshot
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use wasm_bindgen_test :: wasm_bindgen_test ; wasm_bindgen_test :: wasm_bindgen_test_configure ! (run_in_browser) ; # [wasm_bindgen_test] fn drop () { let (.. , receiver) = super :: channel :: < () > () ; assert ! (receiver . receive () . is_none ()) ; } # [wasm_bindgen_test] async fn drop_async () { let (.. , receiver) = super :: channel :: < () > () ; assert ! (receiver . await . is_none ()) ; } }
};
}
