// Generated macro for RustInputsPackager (struct)
macro_rules! Depcrate_compiler_rustRustInputsPackager {
() => {
// Module: crate::compiler::rust
// Provides: {"RustInputsPackager"}
// Dependencies: {}
# [cfg (feature = "dist-client")] struct RustInputsPackager { env_vars : Vec < (OsString , OsString) > , crate_link_paths : Vec < PathBuf > , crate_types : CrateTypes , inputs : Vec < PathBuf > , path_transformer : dist :: PathTransformer , rlib_dep_reader : Option < Arc < RlibDepReader > > , }
};
}
