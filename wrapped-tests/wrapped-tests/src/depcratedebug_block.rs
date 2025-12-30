// Generated macro for debug_block (function)
macro_rules! Depcratedebug_block {
() => {
// Module: crate
// Provides: {"debug_block"}
// Dependencies: {}
# [no_mangle] extern "C-unwind" fn debug_block (block : * mut c_void) { let block : & Block < dyn Fn () > = unsafe { & * (block as * const Block < dyn Fn () >) } ; std :: println ! ("{block:#?}") ; }
};
}
