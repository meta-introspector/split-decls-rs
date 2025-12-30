// Generated macro for wasm_audio_node (function)
macro_rules! Depcrate_wasm_audiowasm_audio_node {
() => {
// Module: crate::wasm_audio
// Provides: {"wasm_audio_node"}
// Dependencies: {}
pub fn wasm_audio_node (ctx : & AudioContext , process : Box < dyn FnMut (& mut [f32]) -> bool > ,) -> Result < AudioWorkletNode , JsValue > { let options = AudioWorkletNodeOptions :: new () ; options . set_processor_options (Some (& js_sys :: Array :: of3 (& wasm_bindgen :: module () , & wasm_bindgen :: memory () , & WasmAudioProcessor (process) . pack () . into () ,))) ; AudioWorkletNode :: new_with_options (ctx , "WasmProcessor" , & options) }
};
}
