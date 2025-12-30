// Generated macro for impl_935 (impl)
macro_rules! Depcrate_features_gen_ClientRectsAndTextsimpl_935 {
() => {
// Module: crate::features::gen_ClientRectsAndTexts
// Provides: {"impl_935"}
// Dependencies: {}
impl ClientRectsAndTexts { # [cfg (feature = "DomRectList")] # [doc = "Construct a new `ClientRectsAndTexts`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"] pub fn new (rect_list : & DomRectList , text_list : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_rect_list (rect_list) ; ret . set_text_list (text_list) ; ret } # [cfg (feature = "DomRectList")] # [deprecated = "Use `set_rect_list()` instead."] pub fn rect_list (& mut self , val : & DomRectList) -> & mut Self { self . set_rect_list (val) ; self } # [deprecated = "Use `set_text_list()` instead."] pub fn text_list (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_text_list (val) ; self } }
};
}
