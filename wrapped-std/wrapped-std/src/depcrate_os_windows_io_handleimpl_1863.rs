// Generated macro for impl_1863 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1863 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1863"}
// Dependencies: {}
impl HandleOrInvalid { # [doc = " Constructs a new instance of `Self` from the given `RawHandle` returned"] # [doc = " from a Windows API that uses `INVALID_HANDLE_VALUE` to indicate"] # [doc = " failure, such as `CreateFileW`."] # [doc = ""] # [doc = " Use `HandleOrNull` instead of `HandleOrInvalid` for APIs that"] # [doc = " use null to indicate failure."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The passed `handle` value must either satisfy the safety requirements"] # [doc = " of [`FromRawHandle::from_raw_handle`], or be"] # [doc = " `INVALID_HANDLE_VALUE` (-1). Note that not all Windows APIs use"] # [doc = " `INVALID_HANDLE_VALUE` for errors; see [here] for the full story."] # [doc = ""] # [doc = " [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443"] # [stable (feature = "io_safety" , since = "1.63.0")] # [inline] pub unsafe fn from_raw_handle (handle : RawHandle) -> Self { Self (handle) } fn is_valid (& self) -> bool { self . 0 != sys :: c :: INVALID_HANDLE_VALUE } }
};
}
