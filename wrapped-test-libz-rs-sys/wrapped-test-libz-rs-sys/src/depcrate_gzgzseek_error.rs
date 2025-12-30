// Generated macro for gzseek_error (function)
macro_rules! Depcrate_gzgzseek_error {
() => {
// Module: crate::gz
// Provides: {"gzseek_error"}
// Dependencies: {}
# [test] fn gzseek_error () { assert_eq ! (unsafe { gzseek (ptr :: null_mut () , 0 , libc :: SEEK_CUR) } , - 1) ; let file = unsafe { gzdopen (- 2 , CString :: new ("w") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; const CONTENT : & [u8] = b"0123456789" ; assert_eq ! (unsafe { gzwrite (file , CONTENT . as_ptr () . cast ::< c_void > () , CONTENT . len () as _) } , CONTENT . len () as _) ; assert_eq ! (unsafe { gzflush (file , Z_SYNC_FLUSH) } , Z_ERRNO) ; assert_eq ! (unsafe { gzseek (file , 0 , libc :: SEEK_CUR) } , - 1) ; unsafe { gzclearerr (file) } ; assert_eq ! (unsafe { gzseek (file , 0 , libc :: SEEK_END) } , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; }
};
}
