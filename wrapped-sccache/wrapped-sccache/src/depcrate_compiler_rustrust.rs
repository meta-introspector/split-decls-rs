// Generated macro for Rust (struct)
macro_rules! Depcrate_compiler_rustRust {
() => {
// Module: crate::compiler::rust
// Provides: {"Rust"}
// Dependencies: {}
# [doc = " A struct on which to hang a `Compiler` impl."] # [derive (Debug , Clone)] pub struct Rust { # [doc = " The path to the rustc executable."] executable : PathBuf , # [doc = " The host triple for this rustc."] host : String , # [doc = " The verbose version for this rustc."] # [doc = ""] # [doc = " Hash calculation will take this version into consideration to prevent"] # [doc = " cached object broken after version bump."] # [doc = ""] # [doc = " Looks like the following:"] # [doc = ""] # [doc = " ```shell"] # [doc = " :) rustc -vV"] # [doc = " rustc 1.66.1 (90743e729 2023-01-10)"] # [doc = " binary: rustc"] # [doc = " commit-hash: 90743e7298aca107ddaa0c202a4d3604e29bfeb6"] # [doc = " commit-date: 2023-01-10"] # [doc = " host: x86_64-unknown-linux-gnu"] # [doc = " release: 1.66.1"] # [doc = " LLVM version: 15.0.2"] # [doc = " ```"] version : String , # [doc = " The path to the rustc sysroot."] sysroot : PathBuf , # [doc = " The digests of all the shared libraries in rustc's $sysroot/lib (or /bin on Windows)."] compiler_shlibs_digests : Vec < String > , # [doc = " A shared, caching reader for rlib dependencies"] # [cfg (feature = "dist-client")] rlib_dep_reader : Option < Arc < RlibDepReader > > , }
};
}
