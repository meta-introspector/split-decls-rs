// Generated macro for Inner (struct)
macro_rules! Depcrate_sync_mpmc_contextInner {
() => {
// Module: crate::sync::mpmc::context
// Provides: {"Inner"}
// Dependencies: {}
# [doc = " Inner representation of `Context`."] # [derive (Debug)] struct Inner { # [doc = " Selected operation."] select : Atomic < usize > , # [doc = " A slot into which another thread may store a pointer to its `Packet`."] packet : Atomic < * mut () > , # [doc = " Thread handle."] thread : Thread , # [doc = " Thread id."] thread_id : usize , }
};
}
