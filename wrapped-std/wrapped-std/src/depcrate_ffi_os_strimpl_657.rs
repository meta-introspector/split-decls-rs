// Generated macro for impl_657 (impl)
macro_rules! Depcrate_ffi_os_strimpl_657 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_657"}
// Dependencies: {}
# [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < OsString > for Rc < OsStr > { # [doc = " Converts an [`OsString`] into an <code>[Rc]<[OsStr]></code> by moving the [`OsString`]"] # [doc = " data into a new [`Rc`] buffer."] # [inline] fn from (s : OsString) -> Rc < OsStr > { let rc = s . inner . into_rc () ; unsafe { Rc :: from_raw (Rc :: into_raw (rc) as * const OsStr) } } }
};
}
