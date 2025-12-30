// Generated macro for impl_1845 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1845 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1845"}
// Dependencies: {}
impl BorrowedHandle < '_ > { # [doc = " Returns a `BorrowedHandle` holding the given raw handle."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The resource pointed to by `handle` must be a valid open handle, it"] # [doc = " must remain open for the duration of the returned `BorrowedHandle`."] # [doc = ""] # [doc = " Note that it *may* have the value `INVALID_HANDLE_VALUE` (-1), which is"] # [doc = " sometimes a valid handle value. See [here] for the full story."] # [doc = ""] # [doc = " And, it *may* have the value `NULL` (0), which can occur when consoles are"] # [doc = " detached from processes, or when `windows_subsystem` is used."] # [doc = ""] # [doc = " [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443"] # [inline] # [rustc_const_stable (feature = "io_safety" , since = "1.63.0")] # [stable (feature = "io_safety" , since = "1.63.0")] pub const unsafe fn borrow_raw (handle : RawHandle) -> Self { Self { handle , _phantom : PhantomData } } }
};
}
