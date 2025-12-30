// Generated macro for TryLockError (enum)
macro_rules! Depcrate_sync_poisonTryLockError {
() => {
// Module: crate::sync::poison
// Provides: {"TryLockError"}
// Dependencies: {}
# [doc = " An enumeration of possible errors associated with a [`TryLockResult`] which"] # [doc = " can occur while trying to acquire a lock, from the [`try_lock`] method on a"] # [doc = " [`Mutex`] or the [`try_read`] and [`try_write`] methods on an [`RwLock`]."] # [doc = ""] # [doc = " [`try_lock`]: crate::sync::Mutex::try_lock"] # [doc = " [`try_read`]: crate::sync::RwLock::try_read"] # [doc = " [`try_write`]: crate::sync::RwLock::try_write"] # [doc = " [`Mutex`]: crate::sync::Mutex"] # [doc = " [`RwLock`]: crate::sync::RwLock"] # [stable (feature = "rust1" , since = "1.0.0")] pub enum TryLockError < T > { # [doc = " The lock could not be acquired because another thread failed while holding"] # [doc = " the lock."] # [stable (feature = "rust1" , since = "1.0.0")] Poisoned (# [stable (feature = "rust1" , since = "1.0.0")] PoisonError < T >) , # [doc = " The lock could not be acquired at this time because the operation would"] # [doc = " otherwise block."] # [stable (feature = "rust1" , since = "1.0.0")] WouldBlock , }
};
}
