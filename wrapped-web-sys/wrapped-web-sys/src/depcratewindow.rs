// Generated macro for window (function)
macro_rules! Depcratewindow {
() => {
// Module: crate
// Provides: {"window"}
// Dependencies: {}
# [doc = " Getter for the `Window` object"] # [doc = ""] # [doc = " [MDN Documentation]"] # [doc = ""] # [doc = " *This API requires the following crate features to be activated: `Window`*"] # [doc = ""] # [doc = " [MDN Documentation]: https://developer.mozilla.org/en-US/docs/Web/API/Window"] # [cfg (feature = "Window")] pub fn window () -> Option < Window > { use wasm_bindgen :: JsCast ; js_sys :: global () . dyn_into :: < Window > () . ok () }
};
}
