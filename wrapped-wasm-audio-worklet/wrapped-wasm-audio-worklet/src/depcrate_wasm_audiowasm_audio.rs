// Generated macro for wasm_audio (function)
macro_rules! Depcrate_wasm_audiowasm_audio {
() => {
// Module: crate::wasm_audio
// Provides: {"wasm_audio"}
// Dependencies: {}
pub async fn wasm_audio (process : Box < dyn FnMut (& mut [f32]) -> bool > ,) -> Result < AudioContext , JsValue > { let ctx = AudioContext :: new () ? ; prepare_wasm_audio (& ctx) . await ? ; let node = wasm_audio_node (& ctx , process) ? ; node . connect_with_audio_node (& ctx . destination ()) ? ; Ok (ctx) }
};
}
