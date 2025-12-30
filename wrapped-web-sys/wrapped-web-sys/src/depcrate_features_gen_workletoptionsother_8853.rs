// Generated macro for other_8853 (other)
macro_rules! Depcrate_features_gen_WorkletOptionsother_8853 {
() => {
// Module: crate::features::gen_WorkletOptions
// Provides: {"other_8853"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WorkletOptions)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `WorkletOptions` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WorkletOptions`*"] pub type WorkletOptions ; # [cfg (feature = "RequestCredentials")] # [doc = "Get the `credentials` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkletOptions`*"] # [wasm_bindgen (method , getter = "credentials")] pub fn get_credentials (this : & WorkletOptions) -> Option < RequestCredentials > ; # [cfg (feature = "RequestCredentials")] # [doc = "Change the `credentials` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkletOptions`*"] # [wasm_bindgen (method , setter = "credentials")] pub fn set_credentials (this : & WorkletOptions , val : RequestCredentials) ; }
};
}
