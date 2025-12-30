// Generated macro for impl_5009 (impl)
macro_rules! Depcrate_features_gen_OfflineAudioCompletionEventInitimpl_5009 {
() => {
// Module: crate::features::gen_OfflineAudioCompletionEventInit
// Provides: {"impl_5009"}
// Dependencies: {}
impl OfflineAudioCompletionEventInit { # [cfg (feature = "AudioBuffer")] # [doc = "Construct a new `OfflineAudioCompletionEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEventInit`*"] pub fn new (rendered_buffer : & AudioBuffer) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_rendered_buffer (rendered_buffer) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "AudioBuffer")] # [deprecated = "Use `set_rendered_buffer()` instead."] pub fn rendered_buffer (& mut self , val : & AudioBuffer) -> & mut Self { self . set_rendered_buffer (val) ; self } }
};
}
