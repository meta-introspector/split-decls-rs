// Generated macro for impl_550 (impl)
macro_rules! Depcrate_web_audio_workletimpl_550 {
() => {
// Module: crate::web::audio_worklet
// Provides: {"impl_550"}
// Dependencies: {}
# [cfg (all (target_family = "wasm" , target_os = "unknown" , feature = "audio-worklet"))] impl AudioWorkletGlobalScopeExt for AudioWorkletGlobalScope { fn register_processor_ext < P > (& self , name : & str) -> Result < () , io :: Error > where P : 'static + ExtendAudioWorkletProcessor , { audio_worklet :: register_processor :: < P > (name) } }
};
}
