// Generated macro for filter_dirs (function)
macro_rules! Depcrate_walkfilter_dirs {
() => {
// Module: crate::walk
// Provides: {"filter_dirs"}
// Dependencies: {}
# [doc = " The default directory filter."] pub fn filter_dirs (path : & Path) -> bool { let skip = ["tidy-test-file" , "compiler/rustc_codegen_cranelift" , "compiler/rustc_codegen_gcc" , "src/llvm-project" , "library/backtrace" , "library/compiler-builtins" , "library/portable-simd" , "library/stdarch" , "src/tools/cargo" , "src/tools/clippy" , "src/tools/libcxx-version" , "src/tools/miri" , "src/tools/rust-analyzer" , "src/tools/rustc-perf" , "src/tools/rustfmt" , "src/tools/enzyme" , "src/doc/book" , "src/doc/edition-guide" , "src/doc/embedded-book" , "src/doc/nomicon" , "src/doc/rust-by-example" , "src/doc/rustc-dev-guide" , "src/doc/reference" , "src/gcc" , "src/bootstrap/target" , "vendor" ,] ; skip . iter () . any (| p | path . ends_with (p)) }
};
}
