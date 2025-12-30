// Generated macro for impl_654 (impl)
macro_rules! Depcrate_ffi_os_strimpl_654 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_654"}
// Dependencies: {}
# [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < OsString > for Arc < OsStr > { # [doc = " Converts an [`OsString`] into an <code>[Arc]<[OsStr]></code> by moving the [`OsString`]"] # [doc = " data into a new [`Arc`] buffer."] # [inline] fn from (s : OsString) -> Arc < OsStr > { let arc = s . inner . into_arc () ; unsafe { Arc :: from_raw (Arc :: into_raw (arc) as * const OsStr) } } }
};
}
