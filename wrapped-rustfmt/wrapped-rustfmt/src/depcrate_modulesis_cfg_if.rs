// Generated macro for is_cfg_if (function)
macro_rules! Depcrate_modulesis_cfg_if {
() => {
// Module: crate::modules
// Provides: {"is_cfg_if"}
// Dependencies: {}
fn is_cfg_if (item : & ast :: Item) -> bool { match item . kind { ast :: ItemKind :: MacCall (ref mac) => { if let Some (first_segment) = mac . path . segments . first () { if first_segment . ident . name == Symbol :: intern ("cfg_if") { return true ; } } false } _ => false , } }
};
}
