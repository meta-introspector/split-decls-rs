// Generated macro for register_thread (function)
macro_rules! Depcrate_thread_unsupported_audio_workletregister_thread {
() => {
// Module: crate::thread::unsupported::audio_worklet
// Provides: {"register_thread"}
// Dependencies: {}
# [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::BaseAudioContextExt::register_thread()`]."] pub (in super :: super) fn register_thread < F > (_ : BaseAudioContext , _ : Option < usize > , _ : F ,) -> RegisterThreadFuture { unreachable ! ("reached `register_thread()` without atomics target feature") }
};
}
