// Generated macro for Clang (struct)
macro_rules! Depcrate_compiler_clangClang {
() => {
// Module: crate::compiler::clang
// Provides: {"Clang"}
// Dependencies: {}
# [doc = " A struct on which to implement `CCompilerImpl`."] # [derive (Clone , Debug)] pub struct Clang { # [doc = " true iff this is clang++."] pub clangplusplus : bool , # [doc = " true iff this is Apple's clang(++)."] pub is_appleclang : bool , # [doc = " String from __VERSION__ macro."] pub version : Option < String > , }
};
}
