// Generated macro for impl_5452 (impl)
macro_rules! Depcrate_features_gen_PresentationConnectionAvailableEventInitimpl_5452 {
() => {
// Module: crate::features::gen_PresentationConnectionAvailableEventInit
// Provides: {"impl_5452"}
// Dependencies: {}
impl PresentationConnectionAvailableEventInit { # [cfg (feature = "PresentationConnection")] # [doc = "Construct a new `PresentationConnectionAvailableEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEventInit`*"] pub fn new (connection : & PresentationConnection) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_connection (connection) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "PresentationConnection")] # [deprecated = "Use `set_connection()` instead."] pub fn connection (& mut self , val : & PresentationConnection) -> & mut Self { self . set_connection (val) ; self } }
};
}
