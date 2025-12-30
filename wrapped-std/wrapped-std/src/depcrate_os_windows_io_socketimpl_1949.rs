// Generated macro for impl_1949 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1949 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1949"}
// Dependencies: {}
impl BorrowedSocket < '_ > { # [doc = " Returns a `BorrowedSocket` holding the given raw socket."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The resource pointed to by `socket` must remain open for the duration of"] # [doc = " the returned `BorrowedSocket`, and it must not have the value"] # [doc = " `INVALID_SOCKET`."] # [inline] # [track_caller] # [rustc_const_stable (feature = "io_safety" , since = "1.63.0")] # [stable (feature = "io_safety" , since = "1.63.0")] pub const unsafe fn borrow_raw (socket : RawSocket) -> Self { Self { socket : ValidRawSocket :: new (socket) . expect ("socket != -1") , _phantom : PhantomData } } }
};
}
