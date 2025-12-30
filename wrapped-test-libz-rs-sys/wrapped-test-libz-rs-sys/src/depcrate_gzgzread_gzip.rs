// Generated macro for gzread_gzip (function)
macro_rules! Depcrate_gzgzread_gzip {
() => {
// Module: crate::gz
// Provides: {"gzread_gzip"}
// Dependencies: {}
# [test] fn gzread_gzip () { const BUF_SIZE : usize = 128 ; const MAX_READ_SIZE : usize = 256 ; let mut buf = [0u8 ; MAX_READ_SIZE] ; let file = unsafe { gzopen (CString :: new (crate_path ("src/test-data/issue-109.gz")) . unwrap () . as_ptr () , CString :: new ("r") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzbuffer (file , BUF_SIZE as c_uint) } , 0) ; let len = c_int :: MAX as c_uint + 1 ; assert_eq ! (unsafe { gzread (file , buf . as_mut_ptr () . cast ::< c_void > () , len as c_uint) } , - 1) ; let mut err = Z_OK ; assert ! (! unsafe { gzerror (file , & mut err as * mut c_int) } . is_null ()) ; assert_eq ! (err , Z_STREAM_ERROR) ; assert_eq ! (unsafe { gzread (file , buf . as_mut_ptr () . cast ::< c_void > () , 1) } , - 1) ; unsafe { gzclearerr (file) } ; assert_eq ! (unsafe { gzread (file , buf . as_mut_ptr () . cast ::< c_void > () , 10) } , 10) ; assert_eq ! (& buf [.. 10] , b"#mtree\n/se") ; let mut bytes_read : usize = 10 ; loop { let ret = unsafe { gzread (file , buf . as_mut_ptr () . cast :: < c_void > () , MAX_READ_SIZE as c_uint ,) } ; assert ! (ret >= 0) ; if ret == 0 { break ; } bytes_read += ret as usize ; } assert_eq ! (bytes_read , 126094) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; }
};
}
