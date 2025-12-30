// Generated macro for other_5302 (other)
macro_rules! Depcrate_features_gen_PermissionDescriptorother_5302 {
() => {
// Module: crate::features::gen_PermissionDescriptor
// Provides: {"other_5302"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PermissionDescriptor)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `PermissionDescriptor` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`*"] pub type PermissionDescriptor ; # [cfg (feature = "PermissionName")] # [doc = "Get the `name` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`, `PermissionName`*"] # [wasm_bindgen (method , getter = "name")] pub fn get_name (this : & PermissionDescriptor) -> PermissionName ; # [cfg (feature = "PermissionName")] # [doc = "Change the `name` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`, `PermissionName`*"] # [wasm_bindgen (method , setter = "name")] pub fn set_name (this : & PermissionDescriptor , val : PermissionName) ; }
};
}
