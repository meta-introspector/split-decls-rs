// Generated macro for impl_5467 (impl)
macro_rules! Depcrate_features_gen_PresentationConnectionCloseEventInitimpl_5467 {
() => {
// Module: crate::features::gen_PresentationConnectionCloseEventInit
// Provides: {"impl_5467"}
// Dependencies: {}
impl PresentationConnectionCloseEventInit { # [cfg (feature = "PresentationConnectionClosedReason")] # [doc = "Construct a new `PresentationConnectionCloseEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`, `PresentationConnectionClosedReason`*"] pub fn new (reason : PresentationConnectionClosedReason) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_reason (reason) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_message()` instead."] pub fn message (& mut self , val : & str) -> & mut Self { self . set_message (val) ; self } # [cfg (feature = "PresentationConnectionClosedReason")] # [deprecated = "Use `set_reason()` instead."] pub fn reason (& mut self , val : PresentationConnectionClosedReason) -> & mut Self { self . set_reason (val) ; self } }
};
}
