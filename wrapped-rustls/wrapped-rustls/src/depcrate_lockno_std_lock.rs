// Generated macro for no_std_lock (module)
macro_rules! Depcrate_lockno_std_lock {
() => {
// Module: crate::lock
// Provides: {"no_std_lock"}
// Dependencies: {}
# [cfg (not (feature = "std"))] mod no_std_lock { use alloc :: boxed :: Box ; use core :: fmt :: Debug ; use core :: ops :: DerefMut ; use crate :: sync :: Arc ; # [doc = " A no-std compatible wrapper around [`Lock`]."] # [derive (Debug)] pub struct Mutex < T > { inner : Arc < dyn Lock < T > > , } impl < T : Send + 'static > Mutex < T > { # [doc = " Creates a new mutex in an unlocked state ready for use."] pub fn new < M > (val : T) -> Self where M : MakeMutex , T : Send + 'static , { Self { inner : M :: make_mutex (val) , } } # [doc = " Acquires the mutex, blocking the current thread until it is able to do so."] # [doc = ""] # [doc = " This will return `None` in the case the mutex is poisoned."] # [inline] pub fn lock (& self) -> Option < MutexGuard < '_ , T > > { self . inner . lock () . ok () } } # [doc = " A lock protecting shared data."] pub trait Lock < T > : Debug + Send + Sync { # [doc = " Acquire the lock."] fn lock (& self) -> Result < MutexGuard < '_ , T > , Poisoned > ; } # [doc = " A lock builder."] pub trait MakeMutex { # [doc = " Create a new mutex."] fn make_mutex < T > (value : T) -> Arc < dyn Lock < T > > where T : Send + 'static ; } # [doc = " A no-std compatible mutex guard."] pub type MutexGuard < 'a , T > = Box < dyn DerefMut < Target = T > + 'a > ; # [doc = " A marker type used to indicate `Lock::lock` failed due to a poisoned lock."] pub struct Poisoned ; }
};
}
