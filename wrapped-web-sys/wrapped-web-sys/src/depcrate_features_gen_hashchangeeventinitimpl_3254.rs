// Generated macro for impl_3254 (impl)
macro_rules! Depcrate_features_gen_HashChangeEventInitimpl_3254 {
() => {
// Module: crate::features::gen_HashChangeEventInit
// Provides: {"impl_3254"}
// Dependencies: {}
impl HashChangeEventInit { # [doc = "Construct a new `HashChangeEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_new_url()` instead."] pub fn new_url (& mut self , val : & str) -> & mut Self { self . set_new_url (val) ; self } # [deprecated = "Use `set_old_url()` instead."] pub fn old_url (& mut self , val : & str) -> & mut Self { self . set_old_url (val) ; self } }
};
}
