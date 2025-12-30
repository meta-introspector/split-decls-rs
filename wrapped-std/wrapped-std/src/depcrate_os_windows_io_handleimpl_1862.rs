// Generated macro for impl_1862 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1862 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1862"}
// Dependencies: {}
impl HandleOrNull { # [doc = " Constructs a new instance of `Self` from the given `RawHandle` returned"] # [doc = " from a Windows API that uses null to indicate failure, such as"] # [doc = " `CreateThread`."] # [doc = ""] # [doc = " Use `HandleOrInvalid` instead of `HandleOrNull` for APIs that"] # [doc = " use `INVALID_HANDLE_VALUE` to indicate failure."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The passed `handle` value must either satisfy the safety requirements"] # [doc = " of [`FromRawHandle::from_raw_handle`], or be null. Note that not all"] # [doc = " Windows APIs use null for errors; see [here] for the full story."] # [doc = ""] # [doc = " [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443"] # [stable (feature = "io_safety" , since = "1.63.0")] # [inline] pub unsafe fn from_raw_handle (handle : RawHandle) -> Self { Self (handle) } fn is_valid (& self) -> bool { ! self . 0 . is_null () } }
};
}
