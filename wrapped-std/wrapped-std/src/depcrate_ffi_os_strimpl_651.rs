// Generated macro for impl_651 (impl)
macro_rules! Depcrate_ffi_os_strimpl_651 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_651"}
// Dependencies: {}
# [stable (feature = "box_from_os_string" , since = "1.20.0")] impl From < OsString > for Box < OsStr > { # [doc = " Converts an [`OsString`] into a <code>[Box]<[OsStr]></code> without copying or allocating."] # [inline] fn from (s : OsString) -> Box < OsStr > { s . into_boxed_os_str () } }
};
}
