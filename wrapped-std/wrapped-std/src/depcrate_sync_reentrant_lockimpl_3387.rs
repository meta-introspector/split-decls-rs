// Generated macro for impl_3387 (impl)
macro_rules! Depcrate_sync_reentrant_lockimpl_3387 {
() => {
// Module: crate::sync::reentrant_lock
// Provides: {"impl_3387"}
// Dependencies: {}
# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T > ReentrantLock < T > { # [doc = " Creates a new re-entrant lock in an unlocked state ready for use."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(reentrant_lock)]"] # [doc = " use std::sync::ReentrantLock;"] # [doc = ""] # [doc = " let lock = ReentrantLock::new(0);"] # [doc = " ```"] pub const fn new (t : T) -> ReentrantLock < T > { ReentrantLock { mutex : sys :: Mutex :: new () , owner : Tid :: new () , lock_count : UnsafeCell :: new (0) , data : t , } } # [doc = " Consumes this lock, returning the underlying data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(reentrant_lock)]"] # [doc = ""] # [doc = " use std::sync::ReentrantLock;"] # [doc = ""] # [doc = " let lock = ReentrantLock::new(0);"] # [doc = " assert_eq!(lock.into_inner(), 0);"] # [doc = " ```"] pub fn into_inner (self) -> T { self . data } }
};
}
