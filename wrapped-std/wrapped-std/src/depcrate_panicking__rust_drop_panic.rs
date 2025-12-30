// Generated macro for __rust_drop_panic (function)
macro_rules! Depcrate_panicking__rust_drop_panic {
() => {
// Module: crate::panicking
// Provides: {"__rust_drop_panic"}
// Dependencies: {}
# [doc = " This function is called by the panic runtime if FFI code catches a Rust"] # [doc = " panic but doesn't rethrow it. We don't support this case since it messes"] # [doc = " with our panic count."] # [cfg (not (test))] # [rustc_std_internal_symbol] extern "C" fn __rust_drop_panic () -> ! { rtabort ! ("Rust panics must be rethrown") ; }
};
}
