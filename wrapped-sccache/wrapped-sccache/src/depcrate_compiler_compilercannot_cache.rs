// Generated macro for cannot_cache (macro)
macro_rules! Depcrate_compiler_compilercannot_cache {
() => {
// Module: crate::compiler::compiler
// Provides: {"cannot_cache"}
// Dependencies: {}
macro_rules ! cannot_cache { ($ why : expr) => { return CompilerArguments :: CannotCache ($ why , None) } ; ($ why : expr , $ extra_info : expr) => { return CompilerArguments :: CannotCache ($ why , Some ($ extra_info)) } ; }
};
}
