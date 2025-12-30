// Generated macro for bench_heap_path_alloc (function)
macro_rules! Depcrate_sys_pal_common_testsbench_heap_path_alloc {
() => {
// Module: crate::sys::pal::common::tests
// Provides: {"bench_heap_path_alloc"}
// Dependencies: {}
# [bench] fn bench_heap_path_alloc (b : & mut test :: Bencher) { let path = repeat ("a") . take (384) . collect :: < String > () ; let p = Path :: new (& path) ; b . iter (| | { run_path_with_cstr (p , & | cstr | { black_box (cstr) ; Ok (()) }) . unwrap () ; }) ; }
};
}
