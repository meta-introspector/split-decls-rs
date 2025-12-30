// Generated macro for Bencher (struct)
macro_rules! Depcrate_benchBencher {
() => {
// Module: crate::bench
// Provides: {"Bencher"}
// Dependencies: {}
# [doc = " Manager of the benchmarking runs."] # [doc = ""] # [doc = " This is fed into functions marked with `#[bench]` to allow for"] # [doc = " set-up & tear-down before running a piece of code repeatedly via a"] # [doc = " call to `iter`."] # [derive (Clone)] pub struct Bencher { mode : BenchMode , summary : Option < stats :: Summary > , pub bytes : u64 , }
};
}
