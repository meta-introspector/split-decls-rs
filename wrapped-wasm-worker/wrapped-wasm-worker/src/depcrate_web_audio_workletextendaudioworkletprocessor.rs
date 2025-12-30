// Generated macro for ExtendAudioWorkletProcessor (trait)
macro_rules! Depcrate_web_audio_workletExtendAudioWorkletProcessor {
() => {
// Module: crate::web::audio_worklet
// Provides: {"ExtendAudioWorkletProcessor"}
// Dependencies: {}
# [doc = " Extends type with [`AudioWorkletProcessor`]."] # [doc = ""] # [doc = " [`AudioWorkletProcessor`]: https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor"] pub trait ExtendAudioWorkletProcessor { # [doc = " Data passed into [`Self::new()`] when using"] # [doc = " [`BaseAudioContextExt::audio_worklet_node()`]."] type Data : 'static + Send ; # [doc = " Equivalent to [`AudioWorkletProcessor()`]."] # [doc = ""] # [doc = " [`AudioWorkletProcessor()`]: https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/AudioWorkletProcessor"] fn new (this : AudioWorkletProcessor , data : Option < Self :: Data > , options : AudioWorkletNodeOptions ,) -> Self ; # [doc = " Equivalent to [`AudioWorkletProcessor.process()`]."] # [doc = ""] # [doc = " [`AudioWorkletProcessor.process()`]: https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/process"] # [allow (unused_variables)] fn process (& mut self , inputs : Array , outputs : Array , parameters : Object) -> bool { false } # [doc = " Equivalent to [`AudioWorkletProcessor.parameterDescriptors`]."] # [doc = ""] # [doc = " [`AudioWorkletProcessor.parameterDescriptors`]: https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/parameterDescriptors"] # [allow (clippy :: must_use_candidate)] fn parameter_descriptors () -> Iterator { Array :: new () . values () } }
};
}
