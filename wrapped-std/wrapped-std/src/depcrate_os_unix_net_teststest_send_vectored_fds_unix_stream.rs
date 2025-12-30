// Generated macro for test_send_vectored_fds_unix_stream (function)
macro_rules! Depcrate_os_unix_net_teststest_send_vectored_fds_unix_stream {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_send_vectored_fds_unix_stream"}
// Dependencies: {}
# [cfg (any (target_os = "android" , target_os = "linux"))] # [test] fn test_send_vectored_fds_unix_stream () { let (s1 , s2) = or_panic ! (UnixStream :: pair ()) ; let buf1 = [1 ; 8] ; let bufs_send = & [IoSlice :: new (& buf1 [..])] [..] ; let mut ancillary1_buffer = [0 ; 128] ; let mut ancillary1 = SocketAncillary :: new (& mut ancillary1_buffer [..]) ; assert ! (ancillary1 . add_fds (& [s1 . as_raw_fd ()] [..])) ; let usize = or_panic ! (s1 . send_vectored_with_ancillary (& bufs_send , & mut ancillary1)) ; assert_eq ! (usize , 8) ; let mut buf2 = [0 ; 8] ; let mut bufs_recv = & mut [IoSliceMut :: new (& mut buf2 [..])] [..] ; let mut ancillary2_buffer = [0 ; 128] ; let mut ancillary2 = SocketAncillary :: new (& mut ancillary2_buffer [..]) ; let usize = or_panic ! (s2 . recv_vectored_with_ancillary (& mut bufs_recv , & mut ancillary2)) ; assert_eq ! (usize , 8) ; assert_eq ! (buf1 , buf2) ; let mut ancillary_data_vec = Vec :: from_iter (ancillary2 . messages ()) ; assert_eq ! (ancillary_data_vec . len () , 1) ; if let AncillaryData :: ScmRights (scm_rights) = ancillary_data_vec . pop () . unwrap () . unwrap () { let fd_vec = Vec :: from_iter (scm_rights) ; assert_eq ! (fd_vec . len () , 1) ; unsafe { libc :: close (fd_vec [0]) ; } } else { unreachable ! ("must be ScmRights") ; } }
};
}
