// Generated macro for io_slice_into_slice (function)
macro_rules! Depcrate_io_testsio_slice_into_slice {
() => {
// Module: crate::io::tests
// Provides: {"io_slice_into_slice"}
// Dependencies: {}
# [test] fn io_slice_into_slice () { let mut buf = [1 ; 8] ; let slice = IoSliceMut :: new (& mut buf) . into_slice () ; assert_eq ! (slice , [1 ; 8]) ; }
};
}
