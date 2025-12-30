// Generated macro for bench_take_read_buf (function)
macro_rules! Depcrate_io_testsbench_take_read_buf {
() => {
// Module: crate::io::tests
// Provides: {"bench_take_read_buf"}
// Dependencies: {}
# [bench] fn bench_take_read_buf (b : & mut test :: Bencher) { b . iter (| | { let buf : & mut [_] = & mut [MaybeUninit :: uninit () ; 64] ; let mut buf : BorrowedBuf < '_ > = buf . into () ; [255 ; 128] . take (64) . read_buf (buf . unfilled ()) . unwrap () ; }) ; }
};
}
