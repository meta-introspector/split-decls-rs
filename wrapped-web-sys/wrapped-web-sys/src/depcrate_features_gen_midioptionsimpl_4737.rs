// Generated macro for impl_4737 (impl)
macro_rules! Depcrate_features_gen_MidiOptionsimpl_4737 {
() => {
// Module: crate::features::gen_MidiOptions
// Provides: {"impl_4737"}
// Dependencies: {}
impl MidiOptions { # [doc = "Construct a new `MidiOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MidiOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_software()` instead."] pub fn software (& mut self , val : bool) -> & mut Self { self . set_software (val) ; self } # [deprecated = "Use `set_sysex()` instead."] pub fn sysex (& mut self , val : bool) -> & mut Self { self . set_sysex (val) ; self } }
};
}
