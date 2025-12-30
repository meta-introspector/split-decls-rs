// Generated macro for is_registered (function)
macro_rules! Depcrate_thread_atomics_audio_workletis_registered {
() => {
// Module: crate::thread::atomics::audio_worklet
// Provides: {"is_registered"}
// Dependencies: {}
# [doc = " Returns [`true`] if this context has a registered thread."] pub (in super :: super) fn is_registered (context : & BaseAudioContext) -> bool { matches ! (context . unchecked_ref ::< BaseAudioContextExt > () . registered () , Some (true)) }
};
}
