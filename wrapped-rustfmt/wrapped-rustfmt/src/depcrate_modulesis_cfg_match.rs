// Generated macro for is_cfg_match (function)
macro_rules! Depcrate_modulesis_cfg_match {
() => {
// Module: crate::modules
// Provides: {"is_cfg_match"}
// Dependencies: {}
fn is_cfg_match (item : & ast :: Item) -> bool { match item . kind { ast :: ItemKind :: MacCall (ref mac) => { if let Some (last_segment) = mac . path . segments . last () { if last_segment . ident . name == Symbol :: intern ("cfg_match") { return true ; } } false } _ => false , } }
};
}
