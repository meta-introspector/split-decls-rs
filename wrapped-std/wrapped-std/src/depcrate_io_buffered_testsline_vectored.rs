// Generated macro for line_vectored (function)
macro_rules! Depcrate_io_buffered_testsline_vectored {
() => {
// Module: crate::io::buffered::tests
// Provides: {"line_vectored"}
// Dependencies: {}
# [test] fn line_vectored () { let mut a = LineWriter :: new (Vec :: new ()) ; assert_eq ! (a . write_vectored (& [IoSlice :: new (& []) , IoSlice :: new (b"\n") , IoSlice :: new (& []) , IoSlice :: new (b"a") ,]) . unwrap () , 2 ,) ; assert_eq ! (a . get_ref () , b"\n") ; assert_eq ! (a . write_vectored (& [IoSlice :: new (& []) , IoSlice :: new (b"b") , IoSlice :: new (& []) , IoSlice :: new (b"a") , IoSlice :: new (& []) , IoSlice :: new (b"c") ,]) . unwrap () , 3 ,) ; assert_eq ! (a . get_ref () , b"\n") ; a . flush () . unwrap () ; assert_eq ! (a . get_ref () , b"\nabac") ; assert_eq ! (a . write_vectored (& []) . unwrap () , 0) ; assert_eq ! (a . write_vectored (& [IoSlice :: new (& []) , IoSlice :: new (& []) , IoSlice :: new (& []) , IoSlice :: new (& []) ,]) . unwrap () , 0 ,) ; assert_eq ! (a . write_vectored (& [IoSlice :: new (b"a\nb") ,]) . unwrap () , 3) ; assert_eq ! (a . get_ref () , b"\nabaca\nb") ; }
};
}
