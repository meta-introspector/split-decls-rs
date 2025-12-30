// Generated macro for other_5090 (other)
macro_rules! Depcrate_features_gen_OvrMultiview2other_5090 {
() => {
// Module: crate::features::gen_OvrMultiview2
// Provides: {"other_5090"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = OVR_multiview2 , typescript_type = "OVR_multiview2")] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `OvrMultiview2` class."] # [doc = ""] # [doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OVR_multiview2)"] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `OvrMultiview2`*"] pub type OvrMultiview2 ; # [cfg (feature = "WebGlTexture")] # [wasm_bindgen (method , structural , js_class = "OVR_multiview2" , js_name = framebufferTextureMultiviewOVR)] # [doc = "The `framebufferTextureMultiviewOVR()` method."] # [doc = ""] # [doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OVR_multiview2/framebufferTextureMultiviewOVR)"] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `OvrMultiview2`, `WebGlTexture`*"] pub fn framebuffer_texture_multiview_ovr (this : & OvrMultiview2 , target : u32 , attachment : u32 , texture : Option < & WebGlTexture > , level : i32 , base_view_index : i32 , num_views : i32 ,) ; }
};
}
