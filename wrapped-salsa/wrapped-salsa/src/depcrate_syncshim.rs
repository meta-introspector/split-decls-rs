// Generated macro for shim (module)
macro_rules! Depcrate_syncshim {
() => {
// Module: crate::sync
// Provides: {"shim"}
// Dependencies: {}
# [cfg (not (feature = "shuttle"))] pub mod shim { pub use parking_lot :: { Mutex , MutexGuard } ; pub use std :: sync :: * ; pub use std :: { thread , thread_local } ; pub mod atomic { pub use portable_atomic :: AtomicU64 ; pub use std :: sync :: atomic :: * ; } # [doc = " A wrapper around parking-lot's `Condvar` to mirror shuttle's API."] pub struct Condvar (parking_lot :: Condvar) ; # [allow (clippy :: derivable_impls)] impl Default for Condvar { fn default () -> Self { Self (Default :: default ()) } } impl std :: fmt :: Debug for Condvar { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_tuple ("Condvar") . field (& self . 0) . finish () } } impl Condvar { pub fn wait < 'a , T > (& self , mut guard : MutexGuard < 'a , T >) -> MutexGuard < 'a , T > { self . 0 . wait (& mut guard) ; guard } pub fn notify_one (& self) { self . 0 . notify_one () ; } pub fn notify_all (& self) { self . 0 . notify_all () ; } } }
};
}
