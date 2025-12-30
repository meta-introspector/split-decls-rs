// Generated macro for create_webgl_context (function)
macro_rules! Depcratecreate_webgl_context {
() => {
// Module: crate
// Provides: {"create_webgl_context"}
// Dependencies: {}
pub fn create_webgl_context (xr_mode : bool) -> Result < WebGl2RenderingContext , JsValue > { let canvas = web_sys :: window () . unwrap () . document () . unwrap () . get_element_by_id ("canvas") . unwrap () . dyn_into :: < HtmlCanvasElement > () . unwrap () ; let gl : WebGl2RenderingContext = if xr_mode { let gl_attribs = Object :: new () ; Reflect :: set (& gl_attribs , & JsValue :: from_str ("xrCompatible") , & JsValue :: TRUE ,) . unwrap () ; canvas . get_context_with_context_options ("webgl2" , & gl_attribs) ? . unwrap () . dyn_into () ? } else { canvas . get_context ("webgl2") ? . unwrap () . dyn_into () ? } ; Ok (gl) }
};
}
