// Generated macro for Command (enum)
macro_rules! Depcrate_thread_atomics_mainCommand {
() => {
// Module: crate::thread::atomics::main
// Provides: {"Command"}
// Dependencies: {}
# [doc = " Command sent to the main thread."] pub (super) enum Command { # [doc = " Spawn a new thread."] Spawn (SpawnData) , # [doc = " Terminate thread."] Terminate { # [doc = " [`ThreadId`] of the thread to be terminated."] id : ThreadId , # [doc = " Value to use `Atomics.waitAsync` on."] value : Pin < Box < AtomicI32 > > , # [doc = " Handle to release thread memory."] memory : ThreadMemory , } , }
};
}
