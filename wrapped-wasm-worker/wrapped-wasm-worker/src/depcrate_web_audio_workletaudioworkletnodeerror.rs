// Generated macro for AudioWorkletNodeError (struct)
macro_rules! Depcrate_web_audio_workletAudioWorkletNodeError {
() => {
// Module: crate::web::audio_worklet
// Provides: {"AudioWorkletNodeError"}
// Dependencies: {}
# [doc = " Error returned by [`BaseAudioContextExt::audio_worklet_node()`]."] pub struct AudioWorkletNodeError < P > where P : ExtendAudioWorkletProcessor , { # [doc = " The passed [`ExtendAudioWorkletProcessor::Data`]."] pub data : P :: Data , # [doc = " The error thrown by [`new AudioWorkletNode`]."] # [doc = ""] # [doc = " [`new AudioWorkletNode`]: https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode"] pub error : io :: Error , }
};
}
