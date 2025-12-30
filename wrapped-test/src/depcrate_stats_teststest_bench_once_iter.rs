// Generated macro for test_bench_once_iter (function)
macro_rules! Depcrate_stats_teststest_bench_once_iter {
() => {
// Module: crate::stats::tests
// Provides: {"test_bench_once_iter"}
// Dependencies: {}
# [test] fn test_bench_once_iter () { fn f (b : & mut Bencher) -> Result < () , String > { b . iter (| | { }) ; Ok (()) } bench :: run_once (f) . unwrap () ; }
};
}
