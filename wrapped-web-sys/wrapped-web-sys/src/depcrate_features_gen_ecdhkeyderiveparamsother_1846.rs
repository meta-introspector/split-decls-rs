// Generated macro for other_1846 (other)
macro_rules! Depcrate_features_gen_EcdhKeyDeriveParamsother_1846 {
() => {
// Module: crate::features::gen_EcdhKeyDeriveParams
// Provides: {"other_1846"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EcdhKeyDeriveParams)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `EcdhKeyDeriveParams` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"] pub type EcdhKeyDeriveParams ; # [doc = "Get the `name` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"] # [wasm_bindgen (method , getter = "name")] pub fn get_name (this : & EcdhKeyDeriveParams) -> :: alloc :: string :: String ; # [doc = "Change the `name` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"] # [wasm_bindgen (method , setter = "name")] pub fn set_name (this : & EcdhKeyDeriveParams , val : & str) ; # [cfg (feature = "CryptoKey")] # [doc = "Get the `public` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CryptoKey`, `EcdhKeyDeriveParams`*"] # [wasm_bindgen (method , getter = "public")] pub fn get_public (this : & EcdhKeyDeriveParams) -> CryptoKey ; # [cfg (feature = "CryptoKey")] # [doc = "Change the `public` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CryptoKey`, `EcdhKeyDeriveParams`*"] # [wasm_bindgen (method , setter = "public")] pub fn set_public (this : & EcdhKeyDeriveParams , val : & CryptoKey) ; }
};
}
