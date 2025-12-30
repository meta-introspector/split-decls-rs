// Generated macro for BorrowedSocket (struct)
macro_rules! Depcrate_os_windows_io_socketBorrowedSocket {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"BorrowedSocket"}
// Dependencies: {}
# [doc = " A borrowed socket."] # [doc = ""] # [doc = " This has a lifetime parameter to tie it to the lifetime of something that"] # [doc = " owns the socket."] # [doc = ""] # [doc = " This uses `repr(transparent)` and has the representation of a host socket,"] # [doc = " so it can be used in FFI in places where a socket is passed as an argument,"] # [doc = " it is not captured or consumed, and it never has the value"] # [doc = " `INVALID_SOCKET`."] # [doc = ""] # [doc = " This type's `.to_owned()` implementation returns another `BorrowedSocket`"] # [doc = " rather than an `OwnedSocket`. It just makes a trivial copy of the raw"] # [doc = " socket, which is then borrowed under the same lifetime."] # [derive (Copy , Clone)] # [repr (transparent)] # [rustc_nonnull_optimization_guaranteed] # [stable (feature = "io_safety" , since = "1.63.0")] pub struct BorrowedSocket < 'socket > { socket : ValidRawSocket , _phantom : PhantomData < & 'socket OwnedSocket > , }
};
}
