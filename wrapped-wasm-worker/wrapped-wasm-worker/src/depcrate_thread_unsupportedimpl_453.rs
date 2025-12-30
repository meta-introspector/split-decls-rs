// Generated macro for impl_453 (impl)
macro_rules! Depcrate_thread_unsupportedimpl_453 {
() => {
// Module: crate::thread::unsupported
// Provides: {"impl_453"}
// Dependencies: {}
impl Scope { # [doc = " Create a [`Scope`]."] # [allow (clippy :: missing_const_for_fn)] pub (super) fn new () -> Self { Self } # [doc = " Returns the number of current threads."] # [allow (clippy :: missing_const_for_fn , clippy :: unused_self)] pub (super) fn thread_count (& self) -> u64 { 0 } # [doc = " End the scope after calling the supplied function."] # [allow (clippy :: missing_const_for_fn , clippy :: unused_self)] pub (super) fn finish (& self) { } # [doc = " End the scope after calling the supplied function."] # [allow (clippy :: missing_const_for_fn , clippy :: unused_self)] pub (super) fn finish_async (& self , _ : & Context < '_ >) -> Poll < () > { Poll :: Ready (()) } }
};
}
