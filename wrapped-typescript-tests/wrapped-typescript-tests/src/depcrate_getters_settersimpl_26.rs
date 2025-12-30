// Generated macro for impl_26 (impl)
macro_rules! Depcrate_getters_settersimpl_26 {
() => {
// Module: crate::getters_setters
// Provides: {"impl_26"}
// Dependencies: {}
# [wasm_bindgen] impl ColorWithSetters { # [wasm_bindgen (constructor)] pub fn new () -> Self { Self { r : 0.0 , _g : 0.0 , _b : 0.0 , a : 0 , } } # [wasm_bindgen (setter)] pub fn set_r (& mut self , r : f64) { self . r = r ; self . a = if self . r > 1.0 { 255 } else if self . r < 0.0 { 0 } else { (self . r * 255.0) as u8 } ; } # [wasm_bindgen (setter)] pub fn set_color_space (_ : String) { } }
};
}
