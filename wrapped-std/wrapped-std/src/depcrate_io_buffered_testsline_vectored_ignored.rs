// Generated macro for line_vectored_ignored (function)
macro_rules! Depcrate_io_buffered_testsline_vectored_ignored {
() => {
// Module: crate::io::buffered::tests
// Provides: {"line_vectored_ignored"}
// Dependencies: {}
# [doc = " Test that, in cases where vectored writing is not enabled, the"] # [doc = " LineWriter uses the normal `write` call, which more-correctly handles"] # [doc = " partial lines"] # [test] fn line_vectored_ignored () { let writer = ProgrammableSink :: default () ; let mut writer = LineWriter :: new (writer) ; let content = [IoSlice :: new (& []) , IoSlice :: new (b"Line 1\nLine") , IoSlice :: new (b" 2\nLine 3\nL") , IoSlice :: new (& []) , IoSlice :: new (& []) , IoSlice :: new (b"ine 4") , IoSlice :: new (b"\nLine 5\n") ,] ; let count = writer . write_vectored (& content) . unwrap () ; assert_eq ! (count , 11) ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\n") ; let count = writer . write_vectored (& content [2 ..]) . unwrap () ; assert_eq ! (count , 11) ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\nLine 2\nLine 3\n") ; let count = writer . write_vectored (& content [5 ..]) . unwrap () ; assert_eq ! (count , 5) ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\nLine 2\nLine 3\n") ; let count = writer . write_vectored (& content [6 ..]) . unwrap () ; assert_eq ! (count , 8) ; assert_eq ! (writer . get_ref () . buffer . as_slice () , b"Line 1\nLine 2\nLine 3\nLine 4\nLine 5\n" . as_ref ()) ; }
};
}
