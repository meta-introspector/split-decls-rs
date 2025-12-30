// Generated macro for std_impl (module)
macro_rules! Depcrate_syncstd_impl {
() => {
// Module: crate::sync
// Provides: {"std_impl"}
// Dependencies: {}
# [cfg (not (feature = "parking_lot"))] mod std_impl { use std :: sync :: { self , PoisonError , TryLockError } ; pub (crate) use std :: sync :: { RwLockReadGuard , RwLockWriteGuard } ; # [derive (Debug)] pub (crate) struct RwLock < T > { inner : sync :: RwLock < T > , } impl < T > RwLock < T > { pub (crate) fn new (val : T) -> Self { Self { inner : sync :: RwLock :: new (val) , } } # [inline] pub (crate) fn get_mut (& mut self) -> & mut T { self . inner . get_mut () . unwrap_or_else (PoisonError :: into_inner) } # [inline] pub (crate) fn read (& self) -> RwLockReadGuard < '_ , T > { self . inner . read () . unwrap_or_else (PoisonError :: into_inner) } # [inline] # [allow (dead_code)] pub (crate) fn try_read (& self) -> Option < RwLockReadGuard < '_ , T > > { match self . inner . try_read () { Ok (guard) => Some (guard) , Err (TryLockError :: Poisoned (e)) => Some (e . into_inner ()) , Err (TryLockError :: WouldBlock) => None , } } # [inline] pub (crate) fn write (& self) -> RwLockWriteGuard < '_ , T > { self . inner . write () . unwrap_or_else (PoisonError :: into_inner) } # [inline] # [allow (dead_code)] pub (crate) fn try_write (& self) -> Option < RwLockWriteGuard < '_ , T > > { match self . inner . try_write () { Ok (guard) => Some (guard) , Err (TryLockError :: Poisoned (e)) => Some (e . into_inner ()) , Err (TryLockError :: WouldBlock) => None , } } } }
};
}
