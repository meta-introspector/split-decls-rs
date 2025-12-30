// Generated macro for last_error (function)
macro_rules! Depcrate_os_windows_io_socketlast_error {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"last_error"}
// Dependencies: {}
# [doc = " Returns the last error from the Windows socket interface."] fn last_error () -> io :: Error { io :: Error :: from_raw_os_error (unsafe { sys :: c :: WSAGetLastError () }) }
};
}
