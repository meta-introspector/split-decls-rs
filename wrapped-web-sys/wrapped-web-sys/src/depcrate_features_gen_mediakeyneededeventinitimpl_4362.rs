// Generated macro for impl_4362 (impl)
macro_rules! Depcrate_features_gen_MediaKeyNeededEventInitimpl_4362 {
() => {
// Module: crate::features::gen_MediaKeyNeededEventInit
// Provides: {"impl_4362"}
// Dependencies: {}
impl MediaKeyNeededEventInit { # [doc = "Construct a new `MediaKeyNeededEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_init_data()` instead."] pub fn init_data (& mut self , val : Option < & :: js_sys :: ArrayBuffer >) -> & mut Self { self . set_init_data (val) ; self } # [deprecated = "Use `set_init_data_type()` instead."] pub fn init_data_type (& mut self , val : & str) -> & mut Self { self . set_init_data_type (val) ; self } }
};
}
