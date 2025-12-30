// Generated macro for sync (module)
macro_rules! Depcrate_loom_stdsync {
() => {
// Module: crate::loom::std
// Provides: {"sync"}
// Dependencies: {}
pub (crate) mod sync { pub (crate) use std :: sync :: { Arc , Weak } ; # [cfg (all (feature = "parking_lot" , not (miri)))] # [allow (unused_imports)] pub (crate) use crate :: loom :: std :: parking_lot :: { Condvar , Mutex , MutexGuard , RwLock , RwLockReadGuard , WaitTimeoutResult , } ; # [cfg (not (all (feature = "parking_lot" , not (miri))))] # [allow (unused_imports)] pub (crate) use std :: sync :: { Condvar , MutexGuard , RwLockReadGuard , WaitTimeoutResult } ; # [cfg (not (all (feature = "parking_lot" , not (miri))))] pub (crate) use crate :: loom :: std :: mutex :: Mutex ; # [cfg (not (all (feature = "parking_lot" , not (miri))))] pub (crate) use crate :: loom :: std :: rwlock :: RwLock ; pub (crate) mod atomic { pub (crate) use crate :: loom :: std :: atomic_u16 :: AtomicU16 ; pub (crate) use crate :: loom :: std :: atomic_u32 :: AtomicU32 ; pub (crate) use crate :: loom :: std :: atomic_u64 :: { AtomicU64 , StaticAtomicU64 } ; pub (crate) use crate :: loom :: std :: atomic_usize :: AtomicUsize ; pub (crate) use std :: sync :: atomic :: { fence , AtomicBool , AtomicPtr , AtomicU8 , Ordering } ; } pub (crate) use super :: barrier :: Barrier ; }
};
}
