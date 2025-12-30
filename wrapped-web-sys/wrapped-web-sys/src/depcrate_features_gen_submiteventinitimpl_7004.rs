// Generated macro for impl_7004 (impl)
macro_rules! Depcrate_features_gen_SubmitEventInitimpl_7004 {
() => {
// Module: crate::features::gen_SubmitEventInit
// Provides: {"impl_7004"}
// Dependencies: {}
impl SubmitEventInit { # [doc = "Construct a new `SubmitEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "HtmlElement")] # [deprecated = "Use `set_submitter()` instead."] pub fn submitter (& mut self , val : Option < & HtmlElement >) -> & mut Self { self . set_submitter (val) ; self } }
};
}
