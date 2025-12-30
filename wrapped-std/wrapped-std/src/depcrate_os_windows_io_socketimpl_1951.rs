// Generated macro for impl_1951 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1951 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1951"}
// Dependencies: {}
impl BorrowedSocket < '_ > { # [doc = " Creates a new `OwnedSocket` instance that shares the same underlying"] # [doc = " object as the existing `BorrowedSocket` instance."] # [stable (feature = "io_safety" , since = "1.63.0")] pub fn try_clone_to_owned (& self) -> io :: Result < OwnedSocket > { let mut info = unsafe { mem :: zeroed :: < sys :: c :: WSAPROTOCOL_INFOW > () } ; let result = unsafe { sys :: c :: WSADuplicateSocketW (self . as_raw_socket () as sys :: c :: SOCKET , sys :: c :: GetCurrentProcessId () , & mut info ,) } ; sys :: net :: cvt (result) ? ; let socket = unsafe { sys :: c :: WSASocketW (info . iAddressFamily , info . iSocketType , info . iProtocol , & info , 0 , sys :: c :: WSA_FLAG_OVERLAPPED | sys :: c :: WSA_FLAG_NO_HANDLE_INHERIT ,) } ; if socket != sys :: c :: INVALID_SOCKET { unsafe { Ok (OwnedSocket :: from_raw_socket (socket as RawSocket)) } } else { let error = unsafe { sys :: c :: WSAGetLastError () } ; if error != sys :: c :: WSAEPROTOTYPE && error != sys :: c :: WSAEINVAL { return Err (io :: Error :: from_raw_os_error (error)) ; } let socket = unsafe { sys :: c :: WSASocketW (info . iAddressFamily , info . iSocketType , info . iProtocol , & info , 0 , sys :: c :: WSA_FLAG_OVERLAPPED ,) } ; if socket == sys :: c :: INVALID_SOCKET { return Err (last_error ()) ; } unsafe { let socket = OwnedSocket :: from_raw_socket (socket as RawSocket) ; socket . set_no_inherit () ? ; Ok (socket) } } } }
};
}
