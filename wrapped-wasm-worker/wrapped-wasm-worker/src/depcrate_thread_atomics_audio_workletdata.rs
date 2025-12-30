// Generated macro for Data (struct)
macro_rules! Depcrate_thread_atomics_audio_workletData {
() => {
// Module: crate::thread::atomics::audio_worklet
// Provides: {"Data"}
// Dependencies: {}
# [doc = " Data stored in [`AudioWorkletNodeOptions.processorOptions`] to transport"] # [doc = " [`ExtendAudioWorkletProcessor::Data`]."] # [doc = ""] # [doc = " [`AudioWorkletNodeOptions.processorOptions`]: https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode#processoroptions"] struct Data { # [doc = " [`TypeId`] to compare to the type when arriving at the constructor."] type_id : TypeId , # [doc = " [`ExtendAudioWorkletProcessor::Data`]."] value : Box < dyn Any > , # [doc = " If [`AudioWorkletNodeOptions.processorOptions`] was empty."] # [doc = ""] # [doc = " [`AudioWorkletNodeOptions.processorOptions`]: https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode#processoroptions"] empty : bool , }
};
}
