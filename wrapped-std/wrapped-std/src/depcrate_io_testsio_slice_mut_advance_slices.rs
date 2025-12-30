// Generated macro for io_slice_mut_advance_slices (function)
macro_rules! Depcrate_io_testsio_slice_mut_advance_slices {
() => {
// Module: crate::io::tests
// Provides: {"io_slice_mut_advance_slices"}
// Dependencies: {}
# [test] fn io_slice_mut_advance_slices () { let mut buf1 = [1 ; 8] ; let mut buf2 = [2 ; 16] ; let mut buf3 = [3 ; 8] ; let mut bufs = & mut [IoSliceMut :: new (& mut buf1) , IoSliceMut :: new (& mut buf2) , IoSliceMut :: new (& mut buf3) ,] [..] ; IoSliceMut :: advance_slices (& mut bufs , 1) ; assert_eq ! (bufs [0] . deref () , [1 ; 7] . as_ref ()) ; assert_eq ! (bufs [1] . deref () , [2 ; 16] . as_ref ()) ; assert_eq ! (bufs [2] . deref () , [3 ; 8] . as_ref ()) ; IoSliceMut :: advance_slices (& mut bufs , 7) ; assert_eq ! (bufs [0] . deref () , [2 ; 16] . as_ref ()) ; assert_eq ! (bufs [1] . deref () , [3 ; 8] . as_ref ()) ; IoSliceMut :: advance_slices (& mut bufs , 18) ; assert_eq ! (bufs [0] . deref () , [3 ; 6] . as_ref ()) ; }
};
}
