// Generated macro for WORKER_LOCK (static)
macro_rules! Depcrate_thread_atomics_audio_worklet_registerWORKER_LOCK {
() => {
// Module: crate::thread::atomics::audio_worklet::register
// Provides: {"WORKER_LOCK"}
// Dependencies: {}
# [doc = " Counts how many workers are currently instantiating."] static WORKER_LOCK : AtomicI32 = AtomicI32 :: new (0) ;
};
}
