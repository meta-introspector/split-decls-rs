// Generated macro for impl_4708 (impl)
macro_rules! Depcrate_features_gen_MidiConnectionEventInitimpl_4708 {
() => {
// Module: crate::features::gen_MidiConnectionEventInit
// Provides: {"impl_4708"}
// Dependencies: {}
impl MidiConnectionEventInit { # [doc = "Construct a new `MidiConnectionEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "MidiPort")] # [deprecated = "Use `set_port()` instead."] pub fn port (& mut self , val : Option < & MidiPort >) -> & mut Self { self . set_port (val) ; self } }
};
}
