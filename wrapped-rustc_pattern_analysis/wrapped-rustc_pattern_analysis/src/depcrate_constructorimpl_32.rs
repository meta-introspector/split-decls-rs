// Generated macro for impl_32 (impl)
macro_rules! Depcrate_constructorimpl_32 {
() => {
// Module: crate::constructor
// Provides: {"impl_32"}
// Dependencies: {}
impl OpaqueId { pub fn new () -> Self { use std :: sync :: atomic :: { AtomicU32 , Ordering } ; static OPAQUE_ID : AtomicU32 = AtomicU32 :: new (0) ; OpaqueId (OPAQUE_ID . fetch_add (1 , Ordering :: SeqCst)) } }
};
}
