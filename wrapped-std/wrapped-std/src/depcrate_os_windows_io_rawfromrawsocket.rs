// Generated macro for FromRawSocket (trait)
macro_rules! Depcrate_os_windows_io_rawFromRawSocket {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"FromRawSocket"}
// Dependencies: {}
# [doc = " Creates I/O objects from raw sockets."] # [stable (feature = "from_raw_os" , since = "1.1.0")] pub trait FromRawSocket { # [doc = " Constructs a new I/O object from the specified raw socket."] # [doc = ""] # [doc = " This function is typically used to **consume ownership** of the socket"] # [doc = " given, passing responsibility for closing the socket to the returned"] # [doc = " object. When used in this way, the returned object"] # [doc = " will take responsibility for closing it when the object goes out of"] # [doc = " scope."] # [doc = ""] # [doc = " However, consuming ownership is not strictly required. Use a"] # [doc = " `From<OwnedSocket>::from` implementation for an API which strictly"] # [doc = " consumes ownership."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `socket` passed in must:"] # [doc = "   - be an [owned socket][io-safety]; in particular, it must be open."] # [doc = "   - be a socket that may be freed via [`closesocket`]."] # [doc = ""] # [doc = " [`closesocket`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-closesocket"] # [doc = " [io-safety]: io#io-safety"] # [stable (feature = "from_raw_os" , since = "1.1.0")] unsafe fn from_raw_socket (sock : RawSocket) -> Self ; }
};
}
