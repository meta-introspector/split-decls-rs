// Generated macro for impl_748 (impl)
macro_rules! Depcrate_syncimpl_748 {
() => {
// Module: crate::sync
// Provides: {"impl_748"}
// Dependencies: {}
impl < T > MTLock < T > { # [inline (always)] pub fn new (inner : T) -> Self { MTLock (Lock :: new (inner)) } # [inline (always)] pub fn into_inner (self) -> T { self . 0 . into_inner () } # [inline (always)] pub fn get_mut (& mut self) -> & mut T { self . 0 . get_mut () } # [inline (always)] pub fn lock (& self) -> LockGuard < '_ , T > { self . 0 . lock () } # [inline (always)] pub fn lock_mut (& self) -> LockGuard < '_ , T > { self . lock () } }
};
}
