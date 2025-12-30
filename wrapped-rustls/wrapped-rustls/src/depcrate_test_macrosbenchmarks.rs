// Generated macro for benchmarks (module)
macro_rules! Depcrate_test_macrosbenchmarks {
() => {
// Module: crate::test_macros
// Provides: {"benchmarks"}
// Dependencies: {}
# [cfg (all (test , bench))] # [macro_rules_attribute :: apply (bench_for_each_provider)] mod benchmarks { # [bench] fn bench_each_provider (b : & mut test :: Bencher) { b . iter (| | super :: provider :: DEFAULT_PROVIDER) ; } }
};
}
