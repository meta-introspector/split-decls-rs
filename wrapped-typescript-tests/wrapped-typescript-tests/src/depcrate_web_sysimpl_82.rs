// Generated macro for impl_82 (impl)
macro_rules! Depcrate_web_sysimpl_82 {
() => {
// Module: crate::web_sys
// Provides: {"impl_82"}
// Dependencies: {}
# [wasm_bindgen] impl DocStruct { pub fn new (doc : Document) -> Self { Self { doc } } pub fn get_doc (& self) -> Document { self . doc . clone () } pub fn append_element (& self , element : & HtmlElement) { self . doc . body () . unwrap () . append_child (element) . unwrap () ; } pub fn append_many (& self , _ : & HtmlElement , _ : & HtmlElement , _ : & HtmlElement) { } }
};
}
