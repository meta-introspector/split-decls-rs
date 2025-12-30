// Generated macro for impl_1950 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1950 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1950"}
// Dependencies: {}
impl OwnedSocket { # [doc = " Creates a new `OwnedSocket` instance that shares the same underlying"] # [doc = " object as the existing `OwnedSocket` instance."] # [stable (feature = "io_safety" , since = "1.63.0")] pub fn try_clone (& self) -> io :: Result < Self > { self . as_socket () . try_clone_to_owned () } # [allow (fuzzy_provenance_casts)] # [cfg (not (target_vendor = "uwp"))] pub (crate) fn set_no_inherit (& self) -> io :: Result < () > { cvt (unsafe { sys :: c :: SetHandleInformation (self . as_raw_socket () as sys :: c :: HANDLE , sys :: c :: HANDLE_FLAG_INHERIT , 0 ,) }) . map (drop) } # [cfg (target_vendor = "uwp")] pub (crate) fn set_no_inherit (& self) -> io :: Result < () > { Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "unavailable on UWP")) } }
};
}
