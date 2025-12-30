// Generated macro for bench_read_to_end (function)
macro_rules! Depcrate_io_testsbench_read_to_end {
() => {
// Module: crate::io::tests
// Provides: {"bench_read_to_end"}
// Dependencies: {}
# [bench] # [cfg_attr (miri , ignore)] fn bench_read_to_end (b : & mut test :: Bencher) { b . iter (| | { let mut lr = repeat (1) . take (10000000) ; let mut vec = Vec :: with_capacity (1024) ; super :: default_read_to_end (& mut lr , & mut vec , None) }) ; }
};
}
