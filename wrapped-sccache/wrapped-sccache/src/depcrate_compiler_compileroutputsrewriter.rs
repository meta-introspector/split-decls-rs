// Generated macro for OutputsRewriter (trait)
macro_rules! Depcrate_compiler_compilerOutputsRewriter {
() => {
// Module: crate::compiler::compiler
// Provides: {"OutputsRewriter"}
// Dependencies: {}
# [cfg (feature = "dist-client")] pub trait OutputsRewriter : Send { # [doc = " Perform any post-compilation handling of outputs, given a Vec of the dist_path and local_path"] fn handle_outputs (self : Box < Self > , path_transformer : & dist :: PathTransformer , output_paths : & [PathBuf] , extra_inputs : & [PathBuf] ,) -> Result < () > ; }
};
}
