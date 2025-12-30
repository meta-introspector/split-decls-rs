// Generated macro for other_6092 (other)
macro_rules! Depcrate_features_gen_RtcFecParametersother_6092 {
() => {
// Module: crate::features::gen_RtcFecParameters
// Provides: {"other_6092"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCFecParameters)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `RtcFecParameters` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"] pub type RtcFecParameters ; # [doc = "Get the `ssrc` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"] # [wasm_bindgen (method , getter = "ssrc")] pub fn get_ssrc (this : & RtcFecParameters) -> Option < u32 > ; # [doc = "Change the `ssrc` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"] # [wasm_bindgen (method , setter = "ssrc")] pub fn set_ssrc (this : & RtcFecParameters , val : u32) ; }
};
}
