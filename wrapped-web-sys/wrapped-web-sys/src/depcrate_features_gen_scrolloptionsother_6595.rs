// Generated macro for other_6595 (other)
macro_rules! Depcrate_features_gen_ScrollOptionsother_6595 {
() => {
// Module: crate::features::gen_ScrollOptions
// Provides: {"other_6595"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ScrollOptions)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `ScrollOptions` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"] pub type ScrollOptions ; # [cfg (feature = "ScrollBehavior")] # [doc = "Get the `behavior` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollOptions`*"] # [wasm_bindgen (method , getter = "behavior")] pub fn get_behavior (this : & ScrollOptions) -> Option < ScrollBehavior > ; # [cfg (feature = "ScrollBehavior")] # [doc = "Change the `behavior` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollOptions`*"] # [wasm_bindgen (method , setter = "behavior")] pub fn set_behavior (this : & ScrollOptions , val : ScrollBehavior) ; }
};
}
