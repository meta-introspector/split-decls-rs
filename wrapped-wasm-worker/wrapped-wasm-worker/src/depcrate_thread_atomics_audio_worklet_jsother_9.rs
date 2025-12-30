// Generated macro for other_9 (other)
macro_rules! Depcrate_thread_atomics_audio_worklet_jsother_9 {
() => {
// Module: crate::thread::atomics::audio_worklet::js
// Provides: {"other_9"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [doc = " Extension for [`BaseAudioContext`](web_sys::BaseAudioContext)."] pub (super) type BaseAudioContextExt ; # [doc = " Returns our custom `registered` property."] # [wasm_bindgen (method , getter , js_name = __web_thread_registered)] pub (super) fn registered (this : & BaseAudioContextExt) -> Option < bool > ; # [doc = " Sets our custom `registered` property."] # [wasm_bindgen (method , setter , js_name = __web_thread_registered)] pub (super) fn set_registered (this : & BaseAudioContextExt , value : bool) ; # [doc = " Type for [`AudioWorkletNodeOptions.processorOptions`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode#processoroptions)."] # [wasm_bindgen (extends = Object)] # [derive (Default)] pub (super) type ProcessorOptions ; # [doc = " Returns our custom `data` property."] # [wasm_bindgen (method , getter , js_name = __web_thread_data)] pub (super) fn data (this : & ProcessorOptions) -> Option < NonNull < Data > > ; # [doc = " Sets our custom `data` property."] # [wasm_bindgen (method , setter , js_name = __web_thread_data)] pub (super) fn set_data (this : & ProcessorOptions , value : NonNull < Data >) ; }
};
}
