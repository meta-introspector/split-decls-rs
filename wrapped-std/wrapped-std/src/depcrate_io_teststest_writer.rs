// Generated macro for test_writer (function)
macro_rules! Depcrate_io_teststest_writer {
() => {
// Module: crate::io::tests
// Provides: {"test_writer"}
// Dependencies: {}
# [doc = " Creates a new writer that reads from at most `n_bufs` and reads"] # [doc = " `per_call` bytes (in total) per call to write."] fn test_writer (n_bufs : usize , per_call : usize) -> TestWriter { TestWriter { n_bufs , per_call , written : Vec :: new () } }
};
}
