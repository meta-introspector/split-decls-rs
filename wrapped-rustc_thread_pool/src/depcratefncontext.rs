// Generated macro for FnContext (struct)
macro_rules! DepcrateFnContext {
() => {
// Module: crate
// Provides: {"FnContext"}
// Dependencies: {}
# [doc = " Provides the calling context to a closure called by `join_context`."] # [derive (Debug)] pub struct FnContext { migrated : bool , # [doc = " disable `Send` and `Sync`, just for a little future-proofing."] _marker : PhantomData < * mut () > , }
};
}
