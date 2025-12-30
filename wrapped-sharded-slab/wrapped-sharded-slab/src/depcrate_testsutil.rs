// Generated macro for util (module)
macro_rules! Depcrate_testsutil {
() => {
// Module: crate::tests
// Provides: {"util"}
// Dependencies: {}
pub (crate) mod util { # [cfg (loom)] use std :: sync :: atomic :: { AtomicUsize , Ordering } ; pub (crate) struct TinyConfig ; impl crate :: Config for TinyConfig { const INITIAL_PAGE_SIZE : usize = 4 ; } # [cfg (loom)] pub (crate) fn run_model (name : & 'static str , f : impl Fn () + Sync + Send + 'static) { run_builder (name , loom :: model :: Builder :: new () , f) } # [cfg (loom)] pub (crate) fn run_builder (name : & 'static str , builder : loom :: model :: Builder , f : impl Fn () + Sync + Send + 'static ,) { let iters = AtomicUsize :: new (1) ; builder . check (move | | { test_println ! ("\n------------ running test {}; iteration {} ------------\n" , name , iters . fetch_add (1 , Ordering :: SeqCst)) ; f () }) ; } }
};
}
