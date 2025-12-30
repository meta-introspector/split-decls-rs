// Generated macro for other_1568 (other)
macro_rules! Depcrate_features_gen_DhKeyDeriveParamsother_1568 {
() => {
// Module: crate::features::gen_DhKeyDeriveParams
// Provides: {"other_1568"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DhKeyDeriveParams)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `DhKeyDeriveParams` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"] pub type DhKeyDeriveParams ; # [doc = "Get the `name` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"] # [wasm_bindgen (method , getter = "name")] pub fn get_name (this : & DhKeyDeriveParams) -> :: alloc :: string :: String ; # [doc = "Change the `name` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"] # [wasm_bindgen (method , setter = "name")] pub fn set_name (this : & DhKeyDeriveParams , val : & str) ; # [cfg (feature = "CryptoKey")] # [doc = "Get the `public` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"] # [wasm_bindgen (method , getter = "public")] pub fn get_public (this : & DhKeyDeriveParams) -> CryptoKey ; # [cfg (feature = "CryptoKey")] # [doc = "Change the `public` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"] # [wasm_bindgen (method , setter = "public")] pub fn set_public (this : & DhKeyDeriveParams , val : & CryptoKey) ; }
};
}
