// Generated macro for other_628 (other)
macro_rules! Depcrate_features_gen_BlobPropertyBagother_628 {
() => {
// Module: crate::features::gen_BlobPropertyBag
// Provides: {"other_628"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BlobPropertyBag)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `BlobPropertyBag` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"] pub type BlobPropertyBag ; # [cfg (feature = "EndingTypes")] # [doc = "Get the `endings` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`, `EndingTypes`*"] # [wasm_bindgen (method , getter = "endings")] pub fn get_endings (this : & BlobPropertyBag) -> Option < EndingTypes > ; # [cfg (feature = "EndingTypes")] # [doc = "Change the `endings` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`, `EndingTypes`*"] # [wasm_bindgen (method , setter = "endings")] pub fn set_endings (this : & BlobPropertyBag , val : EndingTypes) ; # [doc = "Get the `type` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"] # [wasm_bindgen (method , getter = "type")] pub fn get_type (this : & BlobPropertyBag) -> Option < :: alloc :: string :: String > ; # [doc = "Change the `type` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"] # [wasm_bindgen (method , setter = "type")] pub fn set_type (this : & BlobPropertyBag , val : & str) ; }
};
}
