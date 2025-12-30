// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < F : Future > SyncFuture < F > { pub fn new (inner : F) -> Self { Self { inner : SyncWrapper :: new (inner) } } pub fn into_inner (self) -> F { self . inner . into_inner () } }
};
}
