// Generated macro for AsRawHandle (trait)
macro_rules! Depcrate_os_windows_io_rawAsRawHandle {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"AsRawHandle"}
// Dependencies: {}
# [doc = " Extracts raw handles."] # [stable (feature = "rust1" , since = "1.0.0")] pub trait AsRawHandle { # [doc = " Extracts the raw handle."] # [doc = ""] # [doc = " This function is typically used to **borrow** an owned handle."] # [doc = " When used in this way, this method does **not** pass ownership of the"] # [doc = " raw handle to the caller, and the handle is only guaranteed"] # [doc = " to be valid while the original object has not yet been destroyed."] # [doc = ""] # [doc = " This function may return null, such as when called on [`Stdin`],"] # [doc = " [`Stdout`], or [`Stderr`] when the console is detached."] # [doc = ""] # [doc = " However, borrowing is not strictly required. See [`AsHandle::as_handle`]"] # [doc = " for an API which strictly borrows a handle."] # [doc = ""] # [doc = " [`Stdin`]: io::Stdin"] # [doc = " [`Stdout`]: io::Stdout"] # [doc = " [`Stderr`]: io::Stderr"] # [stable (feature = "rust1" , since = "1.0.0")] fn as_raw_handle (& self) -> RawHandle ; }
};
}
