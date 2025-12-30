// Generated macro for run (function)
macro_rules! Depcrate_compiler_interfacerun {
() => {
// Module: crate::compiler_interface
// Provides: {"run"}
// Dependencies: {}
pub (crate) fn run < F , T > (interface : & dyn CompilerInterface , f : F) -> Result < T , Error > where F : FnOnce () -> T , { if TLV . is_set () { Err (Error :: from ("rustc_public already running")) } else { let ptr : * const () = (& raw const interface) as _ ; TLV . set (& Cell :: new (ptr) , | | Ok (f ())) } }
};
}
