// Generated macro for impl_3953 (impl)
macro_rules! Depcrate_features_gen_ImageBitmapOptionsimpl_3953 {
() => {
// Module: crate::features::gen_ImageBitmapOptions
// Provides: {"impl_3953"}
// Dependencies: {}
impl ImageBitmapOptions { # [doc = "Construct a new `ImageBitmapOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "ColorSpaceConversion")] # [deprecated = "Use `set_color_space_conversion()` instead."] pub fn color_space_conversion (& mut self , val : ColorSpaceConversion) -> & mut Self { self . set_color_space_conversion (val) ; self } # [cfg (feature = "ImageOrientation")] # [deprecated = "Use `set_image_orientation()` instead."] pub fn image_orientation (& mut self , val : ImageOrientation) -> & mut Self { self . set_image_orientation (val) ; self } # [cfg (feature = "PremultiplyAlpha")] # [deprecated = "Use `set_premultiply_alpha()` instead."] pub fn premultiply_alpha (& mut self , val : PremultiplyAlpha) -> & mut Self { self . set_premultiply_alpha (val) ; self } # [deprecated = "Use `set_resize_height()` instead."] pub fn resize_height (& mut self , val : u32) -> & mut Self { self . set_resize_height (val) ; self } # [cfg (feature = "ResizeQuality")] # [deprecated = "Use `set_resize_quality()` instead."] pub fn resize_quality (& mut self , val : ResizeQuality) -> & mut Self { self . set_resize_quality (val) ; self } # [deprecated = "Use `set_resize_width()` instead."] pub fn resize_width (& mut self , val : u32) -> & mut Self { self . set_resize_width (val) ; self } }
};
}
