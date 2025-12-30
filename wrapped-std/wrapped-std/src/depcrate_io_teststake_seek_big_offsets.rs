// Generated macro for take_seek_big_offsets (function)
macro_rules! Depcrate_io_teststake_seek_big_offsets {
() => {
// Module: crate::io::tests
// Provides: {"take_seek_big_offsets"}
// Dependencies: {}
# [test] fn take_seek_big_offsets () -> io :: Result < () > { let inner = ExampleHugeRangeOfZeroes { position : 1 } ; let mut take = inner . take (u64 :: MAX - 2) ; assert_eq ! (take . seek (io :: SeekFrom :: Start (u64 :: MAX - 2)) ?, u64 :: MAX - 2) ; assert_eq ! (take . inner . position , u64 :: MAX - 1) ; assert_eq ! (take . seek (io :: SeekFrom :: Start (0)) ?, 0) ; assert_eq ! (take . inner . position , 1) ; assert_eq ! (take . seek (io :: SeekFrom :: End (- 1)) ?, u64 :: MAX - 3) ; assert_eq ! (take . inner . position , u64 :: MAX - 2) ; Ok (()) }
};
}
