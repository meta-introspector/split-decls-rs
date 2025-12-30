// Generated macro for bench_take_read (function)
macro_rules! Depcrate_io_testsbench_take_read {
() => {
// Module: crate::io::tests
// Provides: {"bench_take_read"}
// Dependencies: {}
# [bench] fn bench_take_read (b : & mut test :: Bencher) { b . iter (| | { let mut buf = [0 ; 64] ; [255 ; 128] . take (64) . read (& mut buf) . unwrap () ; }) ; }
};
}
