// Generated macro for maybe_unstable_attr (function)
macro_rules! Depcrate_generatormaybe_unstable_attr {
() => {
// Module: crate::generator
// Provides: {"maybe_unstable_attr"}
// Dependencies: {}
fn maybe_unstable_attr (unstable : bool) -> Option < proc_macro2 :: TokenStream > { if unstable { Some (quote ! { # [cfg (web_sys_unstable_apis)] }) } else { None } }
};
}
