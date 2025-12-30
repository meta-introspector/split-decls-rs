// Generated macro for test_buffered_reader_read_buf (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_reader_read_buf {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_reader_read_buf"}
// Dependencies: {}
# [test] fn test_buffered_reader_read_buf () { let inner : & [u8] = & [5 , 6 , 7 , 0 , 1 , 2 , 3 , 4] ; let mut reader = BufReader :: with_capacity (2 , inner) ; let buf : & mut [_] = & mut [MaybeUninit :: uninit () ; 3] ; let mut buf : BorrowedBuf < '_ > = buf . into () ; reader . read_buf (buf . unfilled ()) . unwrap () ; assert_eq ! (buf . filled () , [5 , 6 , 7]) ; assert_eq ! (reader . buffer () , []) ; let buf : & mut [_] = & mut [MaybeUninit :: uninit () ; 2] ; let mut buf : BorrowedBuf < '_ > = buf . into () ; reader . read_buf (buf . unfilled ()) . unwrap () ; assert_eq ! (buf . filled () , [0 , 1]) ; assert_eq ! (reader . buffer () , []) ; let buf : & mut [_] = & mut [MaybeUninit :: uninit () ; 1] ; let mut buf : BorrowedBuf < '_ > = buf . into () ; reader . read_buf (buf . unfilled ()) . unwrap () ; assert_eq ! (buf . filled () , [2]) ; assert_eq ! (reader . buffer () , [3]) ; let buf : & mut [_] = & mut [MaybeUninit :: uninit () ; 3] ; let mut buf : BorrowedBuf < '_ > = buf . into () ; reader . read_buf (buf . unfilled ()) . unwrap () ; assert_eq ! (buf . filled () , [3]) ; assert_eq ! (reader . buffer () , []) ; reader . read_buf (buf . unfilled ()) . unwrap () ; assert_eq ! (buf . filled () , [3 , 4]) ; assert_eq ! (reader . buffer () , []) ; buf . clear () ; reader . read_buf (buf . unfilled ()) . unwrap () ; assert ! (buf . filled () . is_empty ()) ; }
};
}
