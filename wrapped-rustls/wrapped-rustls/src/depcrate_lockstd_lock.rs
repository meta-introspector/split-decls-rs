// Generated macro for std_lock (module)
macro_rules! Depcrate_lockstd_lock {
() => {
// Module: crate::lock
// Provides: {"std_lock"}
// Dependencies: {}
# [cfg (feature = "std")] mod std_lock { use std :: sync :: Mutex as StdMutex ; pub use std :: sync :: MutexGuard ; # [doc = " A wrapper around [`std::sync::Mutex`]."] # [derive (Debug)] pub struct Mutex < T > { inner : StdMutex < T > , } impl < T > Mutex < T > { # [doc = " Creates a new mutex in an unlocked state ready for use."] pub fn new (data : T) -> Self { Self { inner : StdMutex :: new (data) , } } # [doc = " Acquires the mutex, blocking the current thread until it is able to do so."] # [doc = ""] # [doc = " This will return `None` in the case the mutex is poisoned."] # [inline] pub fn lock (& self) -> Option < MutexGuard < '_ , T > > { self . inner . lock () . ok () } } }
};
}
