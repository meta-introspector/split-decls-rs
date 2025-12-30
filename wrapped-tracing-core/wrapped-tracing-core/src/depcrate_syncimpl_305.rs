// Generated macro for impl_305 (impl)
macro_rules! Depcrate_syncimpl_305 {
() => {
// Module: crate::sync
// Provides: {"impl_305"}
// Dependencies: {}
impl < T > Mutex < T > { pub (crate) fn lock (& self) -> Result < MutexGuard < '_ , T > , () > { Ok (self . inner . lock ()) } }
};
}
