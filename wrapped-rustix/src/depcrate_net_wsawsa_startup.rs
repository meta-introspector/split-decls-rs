// Generated macro for wsa_startup (function)
macro_rules! Depcrate_net_wsawsa_startup {
() => {
// Module: crate::net::wsa
// Provides: {"wsa_startup"}
// Dependencies: {}
# [doc = " `WSAStartup()`—Initialize process-wide Windows support for sockets."] # [doc = ""] # [doc = " On Windows, it's necessary to initialize the sockets subsystem before"] # [doc = " using sockets APIs. The function performs the necessary initialization."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Winsock]"] # [doc = ""] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsastartup"] pub fn wsa_startup () -> io :: Result < WSADATA > { let version = 0x202 ; let mut data = MaybeUninit :: uninit () ; unsafe { let ret = WSAStartup (version , data . as_mut_ptr ()) ; if ret == 0 { Ok (data . assume_init ()) } else { Err (io :: Errno :: from_raw_os_error (WSAGetLastError ())) } } }
};
}
