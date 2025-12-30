// Generated macro for audio_worklet_node (function)
macro_rules! Depcrate_thread_unsupported_audio_workletaudio_worklet_node {
() => {
// Module: crate::thread::unsupported::audio_worklet
// Provides: {"audio_worklet_node"}
// Dependencies: {}
# [doc = " Implementation for"] # [doc = " [`crate::web::audio_worklet::BaseAudioContextExt::audio_worklet_node()`]."] pub (in super :: super) fn audio_worklet_node < P : ExtendAudioWorkletProcessor > (_ : & BaseAudioContext , _ : & str , _ : P :: Data , _ : Option < & AudioWorkletNodeOptions > ,) -> Result < AudioWorkletNode , AudioWorkletNodeError < P > > { unreachable ! ("reached despite not being able to register a thread") }
};
}
