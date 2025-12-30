// Generated macro for ReentrantLockGuard (struct)
macro_rules! Depcrate_sync_reentrant_lockReentrantLockGuard {
() => {
// Module: crate::sync::reentrant_lock
// Provides: {"ReentrantLockGuard"}
// Dependencies: {}
# [doc = " An RAII implementation of a \"scoped lock\" of a re-entrant lock. When this"] # [doc = " structure is dropped (falls out of scope), the lock will be unlocked."] # [doc = ""] # [doc = " The data protected by the mutex can be accessed through this guard via its"] # [doc = " [`Deref`] implementation."] # [doc = ""] # [doc = " This structure is created by the [`lock`](ReentrantLock::lock) method on"] # [doc = " [`ReentrantLock`]."] # [doc = ""] # [doc = " # Mutability"] # [doc = ""] # [doc = " Unlike [`MutexGuard`](super::MutexGuard), `ReentrantLockGuard` does not"] # [doc = " implement [`DerefMut`](crate::ops::DerefMut), because implementation of"] # [doc = " the trait would violate Rust’s reference aliasing rules. Use interior"] # [doc = " mutability (usually [`RefCell`](crate::cell::RefCell)) in order to mutate"] # [doc = " the guarded data."] # [must_use = "if unused the ReentrantLock will immediately unlock"] # [unstable (feature = "reentrant_lock" , issue = "121440")] pub struct ReentrantLockGuard < 'a , T : ? Sized + 'a > { lock : & 'a ReentrantLock < T > , }
};
}
