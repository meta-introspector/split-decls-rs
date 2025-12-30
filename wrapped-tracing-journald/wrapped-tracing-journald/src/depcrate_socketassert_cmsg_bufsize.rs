// Generated macro for assert_cmsg_bufsize (function)
macro_rules! Depcrate_socketassert_cmsg_bufsize {
() => {
// Module: crate::socket
// Provides: {"assert_cmsg_bufsize"}
// Dependencies: {}
fn assert_cmsg_bufsize () { let space_one_fd = unsafe { CMSG_SPACE (size_of :: < RawFd > () as u32) } ; assert ! (space_one_fd <= CMSG_BUFSIZE as u32 , "cmsghdr buffer too small (< {}) to hold a single fd" , space_one_fd) ; }
};
}
