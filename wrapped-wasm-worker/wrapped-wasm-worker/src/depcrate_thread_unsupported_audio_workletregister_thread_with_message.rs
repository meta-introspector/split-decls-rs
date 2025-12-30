// Generated macro for register_thread_with_message (function)
macro_rules! Depcrate_thread_unsupported_audio_workletregister_thread_with_message {
() => {
// Module: crate::thread::unsupported::audio_worklet
// Provides: {"register_thread_with_message"}
// Dependencies: {}
# [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::BaseAudioContextExt::register_thread_with_message()`]."] # [cfg (feature = "message")] pub (in super :: super) fn register_thread_with_message < F , M > (_ : BaseAudioContext , _ : Option < usize > , _ : F , _ : M ,) -> RegisterThreadFuture { unreachable ! ("reached `register_thread()` without atomics target feature") }
};
}
