// Generated macro for WORKSPACES (const)
macro_rules! Depcrate_depsWORKSPACES {
() => {
// Module: crate::deps
// Provides: {"WORKSPACES"}
// Dependencies: {}
# [doc = " The workspaces to check for licensing and optionally permitted dependencies."] # [doc = ""] # [doc = " Each entry consists of a tuple with the following elements:"] # [doc = ""] # [doc = " * The path to the workspace root Cargo.toml file."] # [doc = " * The list of license exceptions."] # [doc = " * Optionally a tuple of:"] # [doc = "     * A list of crates for which dependencies need to be explicitly allowed."] # [doc = "     * The list of allowed dependencies."] # [doc = " * Submodules required for the workspace."] pub (crate) const WORKSPACES : & [(& str , ExceptionList , Option < (& [& str] , & [& str]) > , & [& str])] = & [("." , EXCEPTIONS , Some ((& ["rustc-main"] , PERMITTED_RUSTC_DEPENDENCIES)) , & []) , ("library" , EXCEPTIONS_STDLIB , Some ((& ["sysroot"] , PERMITTED_STDLIB_DEPENDENCIES)) , & []) , ("compiler/rustc_codegen_cranelift" , EXCEPTIONS_CRANELIFT , Some ((& ["rustc_codegen_cranelift"] , PERMITTED_CRANELIFT_DEPENDENCIES)) , & [] ,) , ("compiler/rustc_codegen_gcc" , EXCEPTIONS_GCC , None , & []) , ("src/bootstrap" , EXCEPTIONS_BOOTSTRAP , None , & []) , ("src/tools/cargo" , EXCEPTIONS_CARGO , None , & ["src/tools/cargo"]) , ("src/tools/rust-analyzer" , EXCEPTIONS_RUST_ANALYZER , None , & []) , ("src/tools/rustbook" , EXCEPTIONS_RUSTBOOK , None , & ["src/doc/book" , "src/doc/reference"]) , ("src/tools/rustc-perf" , EXCEPTIONS_RUSTC_PERF , None , & ["src/tools/rustc-perf"]) , ("src/tools/test-float-parse" , EXCEPTIONS , None , & []) , ("tests/run-make-cargo/uefi-qemu/uefi_qemu_test" , EXCEPTIONS_UEFI_QEMU_TEST , None , & []) ,] ;
};
}
