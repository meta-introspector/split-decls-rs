// Generated macro for Msvc (struct)
macro_rules! Depcrate_compiler_msvcMsvc {
() => {
// Module: crate::compiler::msvc
// Provides: {"Msvc"}
// Dependencies: {}
# [doc = " A struct on which to implement `CCompilerImpl`."] # [doc = ""] # [doc = " Needs a little bit of state just to persist `includes_prefix`."] # [derive (Debug , PartialEq , Eq , Clone)] pub struct Msvc { # [doc = " The prefix used in the output of `-showIncludes`."] pub includes_prefix : String , pub is_clang : bool , pub version : Option < String > , }
};
}
