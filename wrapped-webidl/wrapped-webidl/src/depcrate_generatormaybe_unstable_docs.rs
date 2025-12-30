// Generated macro for maybe_unstable_docs (function)
macro_rules! Depcrate_generatormaybe_unstable_docs {
() => {
// Module: crate::generator
// Provides: {"maybe_unstable_docs"}
// Dependencies: {}
fn maybe_unstable_docs (unstable : bool) -> Option < proc_macro2 :: TokenStream > { if unstable { Some (quote ! { # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] }) } else { None } }
};
}
