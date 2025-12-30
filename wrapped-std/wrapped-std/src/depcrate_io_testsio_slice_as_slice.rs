// Generated macro for io_slice_as_slice (function)
macro_rules! Depcrate_io_testsio_slice_as_slice {
() => {
// Module: crate::io::tests
// Provides: {"io_slice_as_slice"}
// Dependencies: {}
# [test] fn io_slice_as_slice () { let buf = [1 ; 8] ; let slice = IoSlice :: new (& buf) . as_slice () ; assert_eq ! (slice , buf) ; }
};
}
