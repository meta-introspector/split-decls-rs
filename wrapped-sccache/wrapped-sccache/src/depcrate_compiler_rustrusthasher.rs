// Generated macro for RustHasher (struct)
macro_rules! Depcrate_compiler_rustRustHasher {
() => {
// Module: crate::compiler::rust
// Provides: {"RustHasher"}
// Dependencies: {}
# [doc = " A struct on which to hang a `CompilerHasher` impl."] # [derive (Debug , Clone)] pub struct RustHasher { # [doc = " The path to the rustc executable, not the rustup proxy."] executable : PathBuf , # [doc = " The host triple for this rustc."] host : String , # [doc = " The version for this rustc."] version : String , # [doc = " The path to the rustc sysroot."] sysroot : PathBuf , # [doc = " The digests of all the shared libraries in rustc's $sysroot/lib (or /bin on Windows)."] compiler_shlibs_digests : Vec < String > , # [doc = " A shared, caching reader for rlib dependencies"] # [cfg (feature = "dist-client")] rlib_dep_reader : Option < Arc < RlibDepReader > > , # [doc = " Parsed arguments from the rustc invocation"] parsed_args : ParsedArguments , }
};
}
