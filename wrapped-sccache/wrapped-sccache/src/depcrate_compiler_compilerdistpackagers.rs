// Generated macro for DistPackagers (type)
macro_rules! Depcrate_compiler_compilerDistPackagers {
() => {
// Module: crate::compiler::compiler
// Provides: {"DistPackagers"}
// Dependencies: {}
# [cfg (feature = "dist-client")] pub type DistPackagers = (Box < dyn pkg :: InputsPackager > , Box < dyn pkg :: ToolchainPackager > , Box < dyn OutputsRewriter > ,) ;
};
}
