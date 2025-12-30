// Generated macro for impl_650 (impl)
macro_rules! Depcrate_ffi_os_strimpl_650 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_650"}
// Dependencies: {}
# [stable (feature = "os_string_from_box" , since = "1.18.0")] impl From < Box < OsStr > > for OsString { # [doc = " Converts a <code>[Box]<[OsStr]></code> into an [`OsString`] without copying or"] # [doc = " allocating."] # [inline] fn from (boxed : Box < OsStr >) -> OsString { boxed . into_os_string () } }
};
}
