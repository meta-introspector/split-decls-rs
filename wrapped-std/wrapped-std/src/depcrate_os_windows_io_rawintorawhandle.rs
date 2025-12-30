// Generated macro for IntoRawHandle (trait)
macro_rules! Depcrate_os_windows_io_rawIntoRawHandle {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"IntoRawHandle"}
// Dependencies: {}
# [doc = " A trait to express the ability to consume an object and acquire ownership of"] # [doc = " its raw `HANDLE`."] # [stable (feature = "into_raw_os" , since = "1.4.0")] pub trait IntoRawHandle { # [doc = " Consumes this object, returning the raw underlying handle."] # [doc = ""] # [doc = " This function is typically used to **transfer ownership** of the underlying"] # [doc = " handle to the caller. When used in this way, callers are then the unique"] # [doc = " owners of the handle and must close it once it's no longer needed."] # [doc = ""] # [doc = " However, transferring ownership is not strictly required. Use a"] # [doc = " `Into<OwnedHandle>::into` implementation for an API which strictly"] # [doc = " transfers ownership."] # [must_use = "losing the raw handle may leak resources"] # [stable (feature = "into_raw_os" , since = "1.4.0")] fn into_raw_handle (self) -> RawHandle ; }
};
}
