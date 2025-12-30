// Generated macro for HandleOrNull (struct)
macro_rules! Depcrate_os_windows_io_handleHandleOrNull {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"HandleOrNull"}
// Dependencies: {}
# [doc = " FFI type for handles in return values or out parameters, where `NULL` is used"] # [doc = " as a sentry value to indicate errors, such as in the return value of `CreateThread`. This uses"] # [doc = " `repr(transparent)` and has the representation of a host handle, so that it can be used in such"] # [doc = " FFI declarations."] # [doc = ""] # [doc = " The only thing you can usefully do with a `HandleOrNull` is to convert it into an"] # [doc = " `OwnedHandle` using its [`TryFrom`] implementation; this conversion takes care of the check for"] # [doc = " `NULL`. This ensures that such FFI calls cannot start using the handle without"] # [doc = " checking for `NULL` first."] # [doc = ""] # [doc = " This type may hold any handle value that [`OwnedHandle`] may hold. As with `OwnedHandle`, when"] # [doc = " it holds `-1`, that value is interpreted as a valid handle value, such as"] # [doc = " [the current process handle], and not `INVALID_HANDLE_VALUE`."] # [doc = ""] # [doc = " If this holds a non-null handle, it will close the handle on drop."] # [doc = ""] # [doc = " [the current process handle]: https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess#remarks"] # [repr (transparent)] # [stable (feature = "io_safety" , since = "1.63.0")] # [derive (Debug)] pub struct HandleOrNull (RawHandle) ;
};
}
