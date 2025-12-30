// Generated macro for gzread_direct (function)
macro_rules! Depcrate_gzgzread_direct {
() => {
// Module: crate::gz
// Provides: {"gzread_direct"}
// Dependencies: {}
# [test] fn gzread_direct () { const BUF_SIZE : usize = 24 ; const MAX_READ_SIZE : usize = 256 ; let mut buf = [0u8 ; MAX_READ_SIZE] ; let path = crate_path ("src/test-data/issue-169.js") ; let file = unsafe { gzopen (CString :: new (path . as_str ()) . unwrap () . as_ptr () , CString :: new ("r") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzbuffer (file , BUF_SIZE as c_uint) } , 0) ; assert_eq ! (unsafe { gzread (file , buf . as_mut_ptr () . cast ::< c_void > () , 20 as c_uint) } , 20 as c_int) ; assert_eq ! (& buf [.. 20] , b"// This file was pro") ; assert_eq ! (unsafe { gzread (file , buf . as_mut_ptr () . cast ::< c_void > () , 0) } , 0) ; assert_eq ! (unsafe { gzread (file , buf . as_mut_ptr () . cast ::< c_void > () , 2 as c_uint) } , 2 as c_int) ; assert_eq ! (& buf [.. 2] , b"ce") ; assert_eq ! (unsafe { gzread (file , buf . as_mut_ptr () . cast ::< c_void > () , 20 as c_uint) } , 20 as c_int) ; assert_eq ! (& buf [.. 20] , b"durally generated fr") ; let mut bytes_read : usize = 20 + 2 + 20 ; loop { let ret = unsafe { gzread (file , buf . as_mut_ptr () . cast :: < c_void > () , MAX_READ_SIZE as c_uint ,) } ; assert ! (ret >= 0) ; if ret == 0 { break ; } bytes_read += ret as usize ; } let Ok (size) = file_size (path . as_str ()) else { panic ! ("Could not find size of file {path}") ; } ; assert_eq ! (bytes_read , size) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; }
};
}
