// Generated macro for futex_wake (function)
macro_rules! Depcrate_thread_atomics_parkerfutex_wake {
() => {
// Module: crate::thread::atomics::parker
// Provides: {"futex_wake"}
// Dependencies: {}
# [doc = " Wake up one thread that's blocked on `futex_wait` on this futex."] # [doc = ""] # [doc = " Returns true if this actually woke up such a thread,"] # [doc = " or false if no thread was waiting on this futex."] pub fn futex_wake (futex : & AtomicU32) -> bool { unsafe { std :: arch :: wasm32 :: memory_atomic_notify (futex as * const AtomicU32 as * mut i32 , 1) > 0 } }
};
}
