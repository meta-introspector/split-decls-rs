// Generated macro for io_slice_advance_slices (function)
macro_rules! Depcrate_io_testsio_slice_advance_slices {
() => {
// Module: crate::io::tests
// Provides: {"io_slice_advance_slices"}
// Dependencies: {}
# [test] fn io_slice_advance_slices () { let buf1 = [1 ; 8] ; let buf2 = [2 ; 16] ; let buf3 = [3 ; 8] ; let mut bufs = & mut [IoSlice :: new (& buf1) , IoSlice :: new (& buf2) , IoSlice :: new (& buf3)] [..] ; IoSlice :: advance_slices (& mut bufs , 1) ; assert_eq ! (bufs [0] . deref () , [1 ; 7] . as_ref ()) ; assert_eq ! (bufs [1] . deref () , [2 ; 16] . as_ref ()) ; assert_eq ! (bufs [2] . deref () , [3 ; 8] . as_ref ()) ; IoSlice :: advance_slices (& mut bufs , 7) ; assert_eq ! (bufs [0] . deref () , [2 ; 16] . as_ref ()) ; assert_eq ! (bufs [1] . deref () , [3 ; 8] . as_ref ()) ; IoSlice :: advance_slices (& mut bufs , 18) ; assert_eq ! (bufs [0] . deref () , [3 ; 6] . as_ref ()) ; }
};
}
