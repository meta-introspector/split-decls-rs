// Generated macro for ThreadMemory (struct)
macro_rules! Depcrate_thread_atomics_memoryThreadMemory {
() => {
// Module: crate::thread::atomics::memory
// Provides: {"ThreadMemory"}
// Dependencies: {}
# [doc = " Holds pointers to the memory of a thread."] # [derive (Debug)] pub (super) struct ThreadMemory { # [doc = " Associated [`ThreadId`]."] thread : ThreadId , # [doc = " TLS memory."] tls_base : f64 , # [doc = " Stack memory."] stack_alloc : f64 , # [doc = " Stack size."] stack_size : Option < usize > , }
};
}
