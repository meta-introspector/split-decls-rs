// Generated macro for io_slice_mut_advance_slices_beyond_total_length (function)
macro_rules! Depcrate_io_testsio_slice_mut_advance_slices_beyond_total_length {
() => {
// Module: crate::io::tests
// Provides: {"io_slice_mut_advance_slices_beyond_total_length"}
// Dependencies: {}
# [test] # [should_panic] fn io_slice_mut_advance_slices_beyond_total_length () { let mut buf1 = [1 ; 8] ; let mut bufs = & mut [IoSliceMut :: new (& mut buf1)] [..] ; IoSliceMut :: advance_slices (& mut bufs , 9) ; assert ! (bufs . is_empty ()) ; }
};
}
