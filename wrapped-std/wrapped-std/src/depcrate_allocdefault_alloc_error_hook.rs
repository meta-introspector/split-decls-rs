// Generated macro for default_alloc_error_hook (function)
macro_rules! Depcrate_allocdefault_alloc_error_hook {
() => {
// Module: crate::alloc
// Provides: {"default_alloc_error_hook"}
// Dependencies: {}
fn default_alloc_error_hook (layout : Layout) { unsafe extern "Rust" { # [rustc_std_internal_symbol] fn __rust_alloc_error_handler_should_panic_v2 () -> u8 ; } if unsafe { __rust_alloc_error_handler_should_panic_v2 () != 0 } { panic ! ("memory allocation of {} bytes failed" , layout . size ()) ; } else { rtprintpanic ! ("memory allocation of {} bytes failed\n" , layout . size ()) ; } }
};
}
