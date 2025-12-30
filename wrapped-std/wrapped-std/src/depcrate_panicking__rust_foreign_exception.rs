// Generated macro for __rust_foreign_exception (function)
macro_rules! Depcrate_panicking__rust_foreign_exception {
() => {
// Module: crate::panicking
// Provides: {"__rust_foreign_exception"}
// Dependencies: {}
# [doc = " This function is called by the panic runtime if it catches an exception"] # [doc = " object which does not correspond to a Rust panic."] # [cfg (not (test))] # [rustc_std_internal_symbol] extern "C" fn __rust_foreign_exception () -> ! { rtabort ! ("Rust cannot catch foreign exceptions") ; }
};
}
