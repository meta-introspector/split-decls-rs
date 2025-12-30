// Generated macro for impl_649 (impl)
macro_rules! Depcrate_ffi_os_strimpl_649 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_649"}
// Dependencies: {}
# [stable (feature = "box_from_cow" , since = "1.45.0")] impl From < Cow < '_ , OsStr > > for Box < OsStr > { # [doc = " Converts a `Cow<'a, OsStr>` into a <code>[Box]&lt;[OsStr]&gt;</code>,"] # [doc = " by copying the contents if they are borrowed."] # [inline] fn from (cow : Cow < '_ , OsStr >) -> Box < OsStr > { match cow { Cow :: Borrowed (s) => Box :: from (s) , Cow :: Owned (s) => Box :: from (s) , } } }
};
}
