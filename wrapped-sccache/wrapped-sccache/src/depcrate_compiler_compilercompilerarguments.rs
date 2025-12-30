// Generated macro for CompilerArguments (enum)
macro_rules! Depcrate_compiler_compilerCompilerArguments {
() => {
// Module: crate::compiler::compiler
// Provides: {"CompilerArguments"}
// Dependencies: {}
# [doc = " Possible results of parsing compiler arguments."] # [derive (Debug , PartialEq , Eq)] pub enum CompilerArguments < T > { # [doc = " Commandline can be handled."] Ok (T) , # [doc = " Cannot cache this compilation."] CannotCache (& 'static str , Option < String >) , # [doc = " This commandline is not a compile."] NotCompilation , }
};
}
