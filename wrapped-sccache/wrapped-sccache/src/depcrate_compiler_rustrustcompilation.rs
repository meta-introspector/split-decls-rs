// Generated macro for RustCompilation (struct)
macro_rules! Depcrate_compiler_rustRustCompilation {
() => {
// Module: crate::compiler::rust
// Provides: {"RustCompilation"}
// Dependencies: {}
# [doc = " A struct on which to hang a `Compilation` impl."] # [derive (Debug , Clone)] pub struct RustCompilation { # [doc = " The path to the rustc executable, not the rustup proxy."] executable : PathBuf , # [doc = " The host triple for this rustc."] host : String , # [doc = " The sysroot for this rustc"] sysroot : PathBuf , # [doc = " A shared, caching reader for rlib dependencies"] # [cfg (feature = "dist-client")] rlib_dep_reader : Option < Arc < RlibDepReader > > , # [doc = " All arguments passed to rustc"] arguments : Vec < Argument < ArgData > > , # [doc = " The compiler inputs."] inputs : Vec < PathBuf > , # [doc = " The compiler outputs."] outputs : HashMap < String , ArtifactDescriptor > , # [doc = " The directories searched for rlibs"] crate_link_paths : Vec < PathBuf > , # [doc = " The crate name being compiled."] crate_name : String , # [doc = " The crate types that will be generated"] crate_types : CrateTypes , # [doc = " If dependency info is being emitted, the name of the dep info file."] dep_info : Option < PathBuf > , # [doc = " The current working directory"] cwd : PathBuf , # [doc = " The environment variables"] env_vars : Vec < (OsString , OsString) > , }
};
}
