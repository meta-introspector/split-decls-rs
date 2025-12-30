// Generated macro for register_processor (function)
macro_rules! Depcrate_thread_unsupported_audio_workletregister_processor {
() => {
// Module: crate::thread::unsupported::audio_worklet
// Provides: {"register_processor"}
// Dependencies: {}
# [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::AudioWorkletGlobalScopeExt::register_processor_ext()`]."] # [allow (clippy :: extra_unused_type_parameters)] pub (in super :: super) fn register_processor < P > (_ : & str) -> Result < () , Error > { unreachable ! ("reached `register_processor()` on the main thread") }
};
}
