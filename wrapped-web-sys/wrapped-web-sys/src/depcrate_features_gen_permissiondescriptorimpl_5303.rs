// Generated macro for impl_5303 (impl)
macro_rules! Depcrate_features_gen_PermissionDescriptorimpl_5303 {
() => {
// Module: crate::features::gen_PermissionDescriptor
// Provides: {"impl_5303"}
// Dependencies: {}
impl PermissionDescriptor { # [cfg (feature = "PermissionName")] # [doc = "Construct a new `PermissionDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`, `PermissionName`*"] pub fn new (name : PermissionName) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret } # [cfg (feature = "PermissionName")] # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : PermissionName) -> & mut Self { self . set_name (val) ; self } }
};
}
