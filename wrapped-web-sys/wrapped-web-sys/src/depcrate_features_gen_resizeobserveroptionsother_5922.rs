// Generated macro for other_5922 (other)
macro_rules! Depcrate_features_gen_ResizeObserverOptionsother_5922 {
() => {
// Module: crate::features::gen_ResizeObserverOptions
// Provides: {"other_5922"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ResizeObserverOptions)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `ResizeObserverOptions` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ResizeObserverOptions`*"] pub type ResizeObserverOptions ; # [cfg (feature = "ResizeObserverBoxOptions")] # [doc = "Get the `box` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ResizeObserverBoxOptions`, `ResizeObserverOptions`*"] # [wasm_bindgen (method , getter = "box")] pub fn get_box (this : & ResizeObserverOptions) -> Option < ResizeObserverBoxOptions > ; # [cfg (feature = "ResizeObserverBoxOptions")] # [doc = "Change the `box` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ResizeObserverBoxOptions`, `ResizeObserverOptions`*"] # [wasm_bindgen (method , setter = "box")] pub fn set_box (this : & ResizeObserverOptions , val : ResizeObserverBoxOptions) ; }
};
}
