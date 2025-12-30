// Generated macro for impl_4309 (impl)
macro_rules! Depcrate_features_gen_MediaElementAudioSourceOptionsimpl_4309 {
() => {
// Module: crate::features::gen_MediaElementAudioSourceOptions
// Provides: {"impl_4309"}
// Dependencies: {}
impl MediaElementAudioSourceOptions { # [cfg (feature = "HtmlMediaElement")] # [doc = "Construct a new `MediaElementAudioSourceOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"] pub fn new (media_element : & HtmlMediaElement) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_media_element (media_element) ; ret } # [cfg (feature = "HtmlMediaElement")] # [deprecated = "Use `set_media_element()` instead."] pub fn media_element (& mut self , val : & HtmlMediaElement) -> & mut Self { self . set_media_element (val) ; self } }
};
}
