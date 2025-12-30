// Generated macro for impl_960 (impl)
macro_rules! Depcrate_features_gen_ClipboardEventInitimpl_960 {
() => {
// Module: crate::features::gen_ClipboardEventInit
// Provides: {"impl_960"}
// Dependencies: {}
impl ClipboardEventInit { # [doc = "Construct a new `ClipboardEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "DataTransfer")] # [deprecated = "Use `set_clipboard_data()` instead."] pub fn clipboard_data (& mut self , val : Option < & DataTransfer >) -> & mut Self { self . set_clipboard_data (val) ; self } }
};
}
