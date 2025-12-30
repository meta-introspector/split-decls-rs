// Generated macro for impl_77 (impl)
macro_rules! Depcrate_patimpl_77 {
() => {
// Module: crate::pat
// Provides: {"impl_77"}
// Dependencies: {}
impl PatId { fn new () -> Self { use std :: sync :: atomic :: { AtomicU32 , Ordering } ; static PAT_ID : AtomicU32 = AtomicU32 :: new (0) ; PatId (PAT_ID . fetch_add (1 , Ordering :: SeqCst)) } }
};
}
