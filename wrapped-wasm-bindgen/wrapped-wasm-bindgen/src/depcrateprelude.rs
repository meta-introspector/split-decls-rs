// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " A module which is typically glob imported."] # [doc = ""] # [doc = " ```"] # [doc = " use wasm_bindgen::prelude::*;"] # [doc = " ```"] pub mod prelude { pub use crate :: closure :: Closure ; pub use crate :: JsCast ; pub use crate :: JsValue ; pub use crate :: UnwrapThrowExt ; # [doc (hidden)] pub use wasm_bindgen_macro :: __wasm_bindgen_class_marker ; pub use wasm_bindgen_macro :: wasm_bindgen ; pub use crate :: JsError ; }
};
}
