// Generated macro for WORKLET_LOCK (static)
macro_rules! Depcrate_thread_atomics_audio_worklet_registerWORKLET_LOCK {
() => {
// Module: crate::thread::atomics::audio_worklet::register
// Provides: {"WORKLET_LOCK"}
// Dependencies: {}
# [doc = " Locks instantiating workers until worklets have finished instantiating."] static WORKLET_LOCK : AtomicI32 = AtomicI32 :: new (0) ;
};
}
