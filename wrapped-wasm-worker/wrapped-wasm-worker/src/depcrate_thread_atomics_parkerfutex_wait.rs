// Generated macro for futex_wait (function)
macro_rules! Depcrate_thread_atomics_parkerfutex_wait {
() => {
// Module: crate::thread::atomics::parker
// Provides: {"futex_wait"}
// Dependencies: {}
# [doc = " Wait for a `futex_wake` operation to wake us."] # [doc = ""] # [doc = " Returns directly if the futex doesn't hold the expected value."] # [doc = ""] # [doc = " Returns false on timeout, and true in all other cases."] pub fn futex_wait (futex : & AtomicU32 , expected : u32 , timeout : Option < Duration >) -> bool { let timeout = timeout . and_then (| t | t . as_nanos () . try_into () . ok ()) . unwrap_or (- 1) ; unsafe { std :: arch :: wasm32 :: memory_atomic_wait32 (futex as * const AtomicU32 as * mut i32 , expected as i32 , timeout ,) < 2 } }
};
}
