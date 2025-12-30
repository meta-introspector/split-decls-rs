// Generated macro for other_4540 (other)
macro_rules! Depcrate_features_gen_MediaStreamAudioSourceOptionsother_4540 {
() => {
// Module: crate::features::gen_MediaStreamAudioSourceOptions
// Provides: {"other_4540"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaStreamAudioSourceOptions)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `MediaStreamAudioSourceOptions` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaStreamAudioSourceOptions`*"] pub type MediaStreamAudioSourceOptions ; # [cfg (feature = "MediaStream")] # [doc = "Get the `mediaStream` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"] # [wasm_bindgen (method , getter = "mediaStream")] pub fn get_media_stream (this : & MediaStreamAudioSourceOptions) -> MediaStream ; # [cfg (feature = "MediaStream")] # [doc = "Change the `mediaStream` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"] # [wasm_bindgen (method , setter = "mediaStream")] pub fn set_media_stream (this : & MediaStreamAudioSourceOptions , val : & MediaStream) ; }
};
}
