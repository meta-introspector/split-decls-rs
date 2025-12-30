// Generated macro for io_slice_mut_advance_slices_empty_slice (function)
macro_rules! Depcrate_io_testsio_slice_mut_advance_slices_empty_slice {
() => {
// Module: crate::io::tests
// Provides: {"io_slice_mut_advance_slices_empty_slice"}
// Dependencies: {}
# [test] # [should_panic] fn io_slice_mut_advance_slices_empty_slice () { let mut empty_bufs = & mut [] [..] ; IoSliceMut :: advance_slices (& mut empty_bufs , 1) ; }
};
}
