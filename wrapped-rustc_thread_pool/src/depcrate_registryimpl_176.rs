// Generated macro for impl_176 (impl)
macro_rules! Depcrate_registryimpl_176 {
() => {
// Module: crate::registry
// Provides: {"impl_176"}
// Dependencies: {}
impl ThreadInfo { fn new (stealer : Stealer < JobRef >) -> ThreadInfo { ThreadInfo { primed : LockLatch :: new () , stopped : LockLatch :: new () , terminate : OnceLatch :: new () , stealer , } } }
};
}
