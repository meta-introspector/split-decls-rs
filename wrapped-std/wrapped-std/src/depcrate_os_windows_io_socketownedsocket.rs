// Generated macro for OwnedSocket (struct)
macro_rules! Depcrate_os_windows_io_socketOwnedSocket {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"OwnedSocket"}
// Dependencies: {}
# [doc = " An owned socket."] # [doc = ""] # [doc = " This closes the socket on drop."] # [doc = ""] # [doc = " This uses `repr(transparent)` and has the representation of a host socket,"] # [doc = " so it can be used in FFI in places where a socket is passed as a consumed"] # [doc = " argument or returned as an owned value, and it never has the value"] # [doc = " `INVALID_SOCKET`."] # [repr (transparent)] # [rustc_nonnull_optimization_guaranteed] # [stable (feature = "io_safety" , since = "1.63.0")] pub struct OwnedSocket { socket : ValidRawSocket , }
};
}
