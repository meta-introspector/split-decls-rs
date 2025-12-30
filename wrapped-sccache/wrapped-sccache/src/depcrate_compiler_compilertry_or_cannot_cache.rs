// Generated macro for try_or_cannot_cache (macro)
macro_rules! Depcrate_compiler_compilertry_or_cannot_cache {
() => {
// Module: crate::compiler::compiler
// Provides: {"try_or_cannot_cache"}
// Dependencies: {}
macro_rules ! try_or_cannot_cache { ($ arg : expr , $ why : expr) => { { match $ arg { Ok (arg) => arg , Err (e) => cannot_cache ! ($ why , e . to_string ()) , } } } ; }
};
}
