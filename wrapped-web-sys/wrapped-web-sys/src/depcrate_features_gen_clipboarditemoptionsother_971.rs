// Generated macro for other_971 (other)
macro_rules! Depcrate_features_gen_ClipboardItemOptionsother_971 {
() => {
// Module: crate::features::gen_ClipboardItemOptions
// Provides: {"other_971"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ClipboardItemOptions)] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `ClipboardItemOptions` dictionary."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`*"] pub type ClipboardItemOptions ; # [cfg (feature = "PresentationStyle")] # [doc = "Get the `presentationStyle` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`, `PresentationStyle`*"] # [wasm_bindgen (method , getter = "presentationStyle")] pub fn get_presentation_style (this : & ClipboardItemOptions) -> Option < PresentationStyle > ; # [cfg (feature = "PresentationStyle")] # [doc = "Change the `presentationStyle` field of this object."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`, `PresentationStyle`*"] # [wasm_bindgen (method , setter = "presentationStyle")] pub fn set_presentation_style (this : & ClipboardItemOptions , val : PresentationStyle) ; }
};
}
