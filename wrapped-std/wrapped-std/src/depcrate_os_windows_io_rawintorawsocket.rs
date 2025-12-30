// Generated macro for IntoRawSocket (trait)
macro_rules! Depcrate_os_windows_io_rawIntoRawSocket {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"IntoRawSocket"}
// Dependencies: {}
# [doc = " A trait to express the ability to consume an object and acquire ownership of"] # [doc = " its raw `SOCKET`."] # [stable (feature = "into_raw_os" , since = "1.4.0")] pub trait IntoRawSocket { # [doc = " Consumes this object, returning the raw underlying socket."] # [doc = ""] # [doc = " This function is typically used to **transfer ownership** of the underlying"] # [doc = " socket to the caller. When used in this way, callers are then the unique"] # [doc = " owners of the socket and must close it once it's no longer needed."] # [doc = ""] # [doc = " However, transferring ownership is not strictly required. Use a"] # [doc = " `Into<OwnedSocket>::into` implementation for an API which strictly"] # [doc = " transfers ownership."] # [must_use = "losing the raw socket may leak resources"] # [stable (feature = "into_raw_os" , since = "1.4.0")] fn into_raw_socket (self) -> RawSocket ; }
};
}
