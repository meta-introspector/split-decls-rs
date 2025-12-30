// Generated macro for OwnedHandle (struct)
macro_rules! Depcrate_os_windows_io_handleOwnedHandle {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"OwnedHandle"}
// Dependencies: {}
# [doc = " An owned handle."] # [doc = ""] # [doc = " This closes the handle on drop."] # [doc = ""] # [doc = " Note that it *may* have the value `-1`, which in `OwnedHandle` always"] # [doc = " represents a valid handle value, such as [the current process handle], and"] # [doc = " not `INVALID_HANDLE_VALUE`, despite the two having the same value. See"] # [doc = " [here] for the full story."] # [doc = ""] # [doc = " And, it *may* have the value `NULL` (0), which can occur when consoles are"] # [doc = " detached from processes, or when `windows_subsystem` is used."] # [doc = ""] # [doc = " `OwnedHandle` uses [`CloseHandle`] to close its handle on drop. As such,"] # [doc = " it must not be used with handles to open registry keys which need to be"] # [doc = " closed with [`RegCloseKey`] instead."] # [doc = ""] # [doc = " [`CloseHandle`]: https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle"] # [doc = " [`RegCloseKey`]: https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey"] # [doc = ""] # [doc = " [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443"] # [doc = " [the current process handle]: https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess#remarks"] # [repr (transparent)] # [stable (feature = "io_safety" , since = "1.63.0")] pub struct OwnedHandle { handle : RawHandle , }
};
}
