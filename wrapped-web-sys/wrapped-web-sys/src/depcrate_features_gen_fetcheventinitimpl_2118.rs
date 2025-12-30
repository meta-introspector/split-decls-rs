// Generated macro for impl_2118 (impl)
macro_rules! Depcrate_features_gen_FetchEventInitimpl_2118 {
() => {
// Module: crate::features::gen_FetchEventInit
// Provides: {"impl_2118"}
// Dependencies: {}
impl FetchEventInit { # [cfg (feature = "Request")] # [doc = "Construct a new `FetchEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FetchEventInit`, `Request`*"] pub fn new (request : & Request) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_request (request) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_client_id()` instead."] pub fn client_id (& mut self , val : Option < & str >) -> & mut Self { self . set_client_id (val) ; self } # [deprecated = "Use `set_is_reload()` instead."] pub fn is_reload (& mut self , val : bool) -> & mut Self { self . set_is_reload (val) ; self } # [cfg (feature = "Request")] # [deprecated = "Use `set_request()` instead."] pub fn request (& mut self , val : & Request) -> & mut Self { self . set_request (val) ; self } }
};
}
