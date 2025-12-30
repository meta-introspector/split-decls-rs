// Generated macro for bench_buffered_reader_small_reads (function)
macro_rules! Depcrate_io_buffered_testsbench_buffered_reader_small_reads {
() => {
// Module: crate::io::buffered::tests
// Provides: {"bench_buffered_reader_small_reads"}
// Dependencies: {}
# [bench] fn bench_buffered_reader_small_reads (b : & mut test :: Bencher) { let data = (0 .. u8 :: MAX) . cycle () . take (1024 * 4) . collect :: < Vec < _ > > () ; b . iter (| | { let mut reader = BufReader :: new (& data [..]) ; let mut buf = [0u8 ; 4] ; for _ in 0 .. 1024 { reader . read_exact (& mut buf) . unwrap () ; core :: hint :: black_box (& buf) ; } }) ; }
};
}
