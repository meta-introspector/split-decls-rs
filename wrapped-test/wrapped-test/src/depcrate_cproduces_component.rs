// Generated macro for produces_component (function)
macro_rules! Depcrate_cproduces_component {
() => {
// Module: crate::c
// Provides: {"produces_component"}
// Dependencies: {}
fn produces_component (runner : & Runner < '_ >) -> bool { match runner . opts . c . c_target . as_str () { "wasm32-wasip1" => false , _ => true , } }
};
}
