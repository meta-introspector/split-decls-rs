// Generated macro for get_path_mappings (function)
macro_rules! Depcrate_compiler_rustget_path_mappings {
() => {
// Module: crate::compiler::rust
// Provides: {"get_path_mappings"}
// Dependencies: {}
# [cfg (feature = "dist-client")] fn get_path_mappings (path_transformer : & dist :: PathTransformer ,) -> impl Iterator < Item = (PathBuf , String) > { path_transformer . disk_mappings () }
};
}
