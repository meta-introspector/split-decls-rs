// Generated macro for with_ctx (function)
macro_rules! Depcrate_panic_contextwith_ctx {
() => {
// Module: crate::panic_context
// Provides: {"with_ctx"}
// Dependencies: {}
fn with_ctx (f : impl FnOnce (& mut Vec < String >)) { thread_local ! { static CTX : RefCell < Vec < String >> = const { RefCell :: new (Vec :: new ()) } ; } CTX . with (| ctx | f (& mut ctx . borrow_mut ())) ; }
};
}
