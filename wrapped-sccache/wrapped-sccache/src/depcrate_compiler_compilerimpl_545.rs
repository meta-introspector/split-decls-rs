// Generated macro for impl_545 (impl)
macro_rules! Depcrate_compiler_compilerimpl_545 {
() => {
// Module: crate::compiler::compiler
// Provides: {"impl_545"}
// Dependencies: {}
impl < I > CCompileCommand < I > where I : CompileCommandImpl , { # [allow (clippy :: new_ret_no_self)] pub fn new < T > (cmd : I) -> Box < dyn CompileCommand < T > > where T : CommandCreatorSync , { Box :: new (CCompileCommand { cmd }) as Box < dyn CompileCommand < T > > } }
};
}
