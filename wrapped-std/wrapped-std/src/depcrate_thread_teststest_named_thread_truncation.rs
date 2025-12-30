// Generated macro for test_named_thread_truncation (function)
macro_rules! Depcrate_thread_teststest_named_thread_truncation {
() => {
// Module: crate::thread::tests
// Provides: {"test_named_thread_truncation"}
// Dependencies: {}
# [cfg (any (all (target_os = "linux" , target_env = "gnu") , target_vendor = "apple" ,))] # [test] fn test_named_thread_truncation () { use crate :: ffi :: CStr ; let long_name = crate :: iter :: once ("test_named_thread_truncation") . chain (crate :: iter :: repeat (" yada") . take (100)) . collect :: < String > () ; let result = Builder :: new () . name (long_name . clone ()) . spawn (move | | { assert_eq ! (thread :: current () . name () , Some (long_name . as_str ())) ; let mut buf = vec ! [0u8 ; long_name . len () + 1] ; unsafe { libc :: pthread_getname_np (libc :: pthread_self () , buf . as_mut_ptr () . cast () , buf . len ()) ; } let cstr = CStr :: from_bytes_until_nul (& buf) . unwrap () ; assert ! (cstr . to_bytes () . len () > 0) ; assert ! (long_name . as_bytes () . starts_with (cstr . to_bytes ())) ; }) ; result . unwrap () . join () . unwrap () ; }
};
}
