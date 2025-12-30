// Generated macro for AsRawSocket (trait)
macro_rules! Depcrate_os_windows_io_rawAsRawSocket {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"AsRawSocket"}
// Dependencies: {}
# [doc = " Extracts raw sockets."] # [stable (feature = "rust1" , since = "1.0.0")] pub trait AsRawSocket { # [doc = " Extracts the raw socket."] # [doc = ""] # [doc = " This function is typically used to **borrow** an owned socket."] # [doc = " When used in this way, this method does **not** pass ownership of the"] # [doc = " raw socket to the caller, and the socket is only guaranteed"] # [doc = " to be valid while the original object has not yet been destroyed."] # [doc = ""] # [doc = " However, borrowing is not strictly required. See [`AsSocket::as_socket`]"] # [doc = " for an API which strictly borrows a socket."] # [stable (feature = "rust1" , since = "1.0.0")] fn as_raw_socket (& self) -> RawSocket ; }
};
}
