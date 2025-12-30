// Generated macro for prepare_wasm_audio (function)
macro_rules! Depcrate_wasm_audioprepare_wasm_audio {
() => {
// Module: crate::wasm_audio
// Provides: {"prepare_wasm_audio"}
// Dependencies: {}
pub async fn prepare_wasm_audio (ctx : & AudioContext) -> Result < () , JsValue > { let mod_url = wasm_bindgen :: link_to ! (module = "/src/worklet.js") ; JsFuture :: from (ctx . audio_worklet () ? . add_module (& mod_url) ?) . await ? ; Ok (()) }
};
}
