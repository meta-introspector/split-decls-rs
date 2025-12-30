// Generated macro for other_5321 (other)
macro_rules! Depcrate_features_gen_Permissionsother_5321 {
() => {
// Module: crate::features::gen_Permissions
// Provides: {"other_5321"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Permissions , typescript_type = "Permissions")] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `Permissions` class."] # [doc = ""] # [doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions)"] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `Permissions`*"] pub type Permissions ; # [wasm_bindgen (catch , method , structural , js_class = "Permissions" , js_name = query)] # [doc = "The `query()` method."] # [doc = ""] # [doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/query)"] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `Permissions`*"] pub fn query (this : & Permissions , permission : & :: js_sys :: Object ,) -> Result < :: js_sys :: Promise , JsValue > ; # [wasm_bindgen (catch , method , structural , js_class = "Permissions" , js_name = revoke)] # [doc = "The `revoke()` method."] # [doc = ""] # [doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/revoke)"] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `Permissions`*"] pub fn revoke (this : & Permissions , permission : & :: js_sys :: Object ,) -> Result < :: js_sys :: Promise , JsValue > ; }
};
}
