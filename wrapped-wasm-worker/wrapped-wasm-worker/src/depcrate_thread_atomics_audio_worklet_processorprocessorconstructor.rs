// Generated macro for ProcessorConstructor (trait)
macro_rules! Depcrate_thread_atomics_audio_worklet_processorProcessorConstructor {
() => {
// Module: crate::thread::atomics::audio_worklet::processor
// Provides: {"ProcessorConstructor"}
// Dependencies: {}
# [doc = " Object-safe version of [`ExtendAudioWorkletProcessor`]."] trait ProcessorConstructor { # [doc = " Calls the underlying [`ExtendAudioWorkletProcessor::new`]."] fn instantiate (& mut self , this : web_sys :: AudioWorkletProcessor , options : AudioWorkletNodeOptions ,) -> __WebThreadProcessor ; # [doc = " Calls the underlying"] # [doc = " [`ExtendAudioWorkletProcessor::parameter_descriptors`]."] fn parameter_descriptors (& self) -> Iterator ; }
};
}
