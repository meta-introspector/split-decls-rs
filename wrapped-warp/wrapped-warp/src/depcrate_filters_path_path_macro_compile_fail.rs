// Generated macro for _path_macro_compile_fail (function)
macro_rules! Depcrate_filters_path_path_macro_compile_fail {
() => {
// Module: crate::filters::path
// Provides: {"_path_macro_compile_fail"}
// Dependencies: {}
# [doc = " ```compile_fail"] # [doc = " warp::path!(\"foo\" / .. / \"bar\");"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " warp::path!(.. / \"bar\");"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " warp::path!(\"foo\" ..);"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " warp::path!(\"foo\" / .. /);"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " warp::path!(..);"] # [doc = " ```"] fn _path_macro_compile_fail () { }
};
}
