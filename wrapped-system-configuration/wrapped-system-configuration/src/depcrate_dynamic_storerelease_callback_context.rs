// Generated macro for release_callback_context (function)
macro_rules! Depcrate_dynamic_storerelease_callback_context {
() => {
// Module: crate::dynamic_store
// Provides: {"release_callback_context"}
// Dependencies: {}
unsafe extern "C" fn release_callback_context < T > (context_ptr : * const c_void) { let _context = Box :: from_raw (context_ptr as * mut SCDynamicStoreCallBackContext < T >) ; }
};
}
