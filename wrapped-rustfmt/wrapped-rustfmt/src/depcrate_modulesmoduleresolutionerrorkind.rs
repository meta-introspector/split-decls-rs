// Generated macro for ModuleResolutionErrorKind (enum)
macro_rules! Depcrate_modulesModuleResolutionErrorKind {
() => {
// Module: crate::modules
// Provides: {"ModuleResolutionErrorKind"}
// Dependencies: {}
# [doc = " Defines variants similar to those of [rustc_expand::module::ModError]"] # [derive (Debug , Error)] pub (crate) enum ModuleResolutionErrorKind { # [doc = " Find a file that cannot be parsed."] # [error ("cannot parse {file}")] ParseError { file : PathBuf } , # [doc = " File cannot be found."] # [error ("{file} does not exist")] NotFound { file : PathBuf } , # [doc = " File a.rs and a/mod.rs both exist"] # [error ("file for module found at both {default_path:?} and {secondary_path:?}")] MultipleCandidates { default_path : PathBuf , secondary_path : PathBuf , } , }
};
}
