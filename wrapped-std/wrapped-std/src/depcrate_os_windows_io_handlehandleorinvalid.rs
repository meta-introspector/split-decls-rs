// Generated macro for HandleOrInvalid (struct)
macro_rules! Depcrate_os_windows_io_handleHandleOrInvalid {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"HandleOrInvalid"}
// Dependencies: {}
# [doc = " FFI type for handles in return values or out parameters, where `INVALID_HANDLE_VALUE` is used"] # [doc = " as a sentry value to indicate errors, such as in the return value of `CreateFileW`. This uses"] # [doc = " `repr(transparent)` and has the representation of a host handle, so that it can be used in such"] # [doc = " FFI declarations."] # [doc = ""] # [doc = " The only thing you can usefully do with a `HandleOrInvalid` is to convert it into an"] # [doc = " `OwnedHandle` using its [`TryFrom`] implementation; this conversion takes care of the check for"] # [doc = " `INVALID_HANDLE_VALUE`. This ensures that such FFI calls cannot start using the handle without"] # [doc = " checking for `INVALID_HANDLE_VALUE` first."] # [doc = ""] # [doc = " This type may hold any handle value that [`OwnedHandle`] may hold, except that when it holds"] # [doc = " `-1`, that value is interpreted to mean `INVALID_HANDLE_VALUE`."] # [doc = ""] # [doc = " If holds a handle other than `INVALID_HANDLE_VALUE`, it will close the handle on drop."] # [repr (transparent)] # [stable (feature = "io_safety" , since = "1.63.0")] # [derive (Debug)] pub struct HandleOrInvalid (RawHandle) ;
};
}
