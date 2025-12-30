// Generated macro for impl_1679 (impl)
macro_rules! Depcrate_systemimpl_1679 {
() => {
// Module: crate::system
// Provides: {"impl_1679"}
// Dependencies: {}
impl Uname { # [doc = " `sysname`—Operating system release name."] # [inline] pub fn sysname (& self) -> & CStr { Self :: to_cstr (self . 0 . sysname . as_ptr () . cast ()) } # [doc = " `nodename`—Name with vague meaning."] # [doc = ""] # [doc = " This is intended to be a network name, however it's unable to convey"] # [doc = " information about hosts that have multiple names, or any information"] # [doc = " about where the names are visible."] # [doc = ""] # [doc = " This corresponds to the `gethostname` value."] # [inline] pub fn nodename (& self) -> & CStr { Self :: to_cstr (self . 0 . nodename . as_ptr () . cast ()) } # [doc = " `release`—Operating system release version string."] # [inline] pub fn release (& self) -> & CStr { Self :: to_cstr (self . 0 . release . as_ptr () . cast ()) } # [doc = " `version`—Operating system build identifiers."] # [inline] pub fn version (& self) -> & CStr { Self :: to_cstr (self . 0 . version . as_ptr () . cast ()) } # [doc = " `machine`—Hardware architecture identifier."] # [inline] pub fn machine (& self) -> & CStr { Self :: to_cstr (self . 0 . machine . as_ptr () . cast ()) } # [doc = " `domainname`—NIS or YP domain identifier."] # [cfg (linux_kernel)] # [inline] pub fn domainname (& self) -> & CStr { Self :: to_cstr (self . 0 . domainname . as_ptr () . cast ()) } # [inline] fn to_cstr < 'a > (ptr : * const u8) -> & 'a CStr { unsafe { CStr :: from_ptr (ptr . cast ()) } } }
};
}
