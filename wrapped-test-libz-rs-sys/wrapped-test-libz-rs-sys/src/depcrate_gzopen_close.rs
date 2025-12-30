// Generated macro for open_close (function)
macro_rules! Depcrate_gzopen_close {
() => {
// Module: crate::gz
// Provides: {"open_close"}
// Dependencies: {}
# [test] fn open_close () { test_open ! (crate_path ("src/test-data/issue-109.gz") , "r" , true) ; test_open ! (crate_path ("src/test-data/issue-109.gz") , "rb" , true) ; test_open ! (crate_path ("src/test-data/issue-109.gz") , "" , false) ; test_open ! (crate_path ("src/test-data/issue-109.gz") , "e" , false) ; test_open ! (crate_path ("src/test-data/issue-109.gz") , "+" , false) ; test_open ! (crate_path ("src/test-data/issue-109.gz") , "Tr" , false) ; test_open ! (crate_path ("src/test-data/no-such-file.gz") , "r" , false) ; assert_eq ! (unsafe { gzclose (ptr :: null_mut ()) } , Z_STREAM_ERROR) ; let cpath = CString :: new (crate_path ("src/test-data/issue-109.gz")) . unwrap () ; let fd = unsafe { libc :: open (cpath . as_ptr () , libc :: O_RDONLY) } ; assert_ne ! (fd , - 1) ; test_fdopen ! (fd , "r" , true) ; }
};
}
