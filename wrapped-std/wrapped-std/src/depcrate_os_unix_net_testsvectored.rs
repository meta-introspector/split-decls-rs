// Generated macro for vectored (function)
macro_rules! Depcrate_os_unix_net_testsvectored {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"vectored"}
// Dependencies: {}
# [test] fn vectored () { let (mut s1 , mut s2) = or_panic ! (UnixStream :: pair ()) ; let len = or_panic ! (s1 . write_vectored (& [IoSlice :: new (b"hello") , IoSlice :: new (b" ") , IoSlice :: new (b"world!")] ,)) ; assert_eq ! (len , 12) ; let mut buf1 = [0 ; 6] ; let mut buf2 = [0 ; 7] ; let len = or_panic ! (s2 . read_vectored (& mut [IoSliceMut :: new (& mut buf1) , IoSliceMut :: new (& mut buf2)] ,)) ; assert_eq ! (len , 12) ; assert_eq ! (& buf1 , b"hello ") ; assert_eq ! (& buf2 , b"world!\0") ; }
};
}
