// Generated macro for impl_6945 (impl)
macro_rules! Depcrate_features_gen_StorageEventInitimpl_6945 {
() => {
// Module: crate::features::gen_StorageEventInit
// Provides: {"impl_6945"}
// Dependencies: {}
impl StorageEventInit { # [doc = "Construct a new `StorageEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_key()` instead."] pub fn key (& mut self , val : Option < & str >) -> & mut Self { self . set_key (val) ; self } # [deprecated = "Use `set_new_value()` instead."] pub fn new_value (& mut self , val : Option < & str >) -> & mut Self { self . set_new_value (val) ; self } # [deprecated = "Use `set_old_value()` instead."] pub fn old_value (& mut self , val : Option < & str >) -> & mut Self { self . set_old_value (val) ; self } # [cfg (feature = "Storage")] # [deprecated = "Use `set_storage_area()` instead."] pub fn storage_area (& mut self , val : Option < & Storage >) -> & mut Self { self . set_storage_area (val) ; self } # [deprecated = "Use `set_url()` instead."] pub fn url (& mut self , val : & str) -> & mut Self { self . set_url (val) ; self } }
};
}
