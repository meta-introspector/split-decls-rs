// Generated macro for impl_20 (impl)
macro_rules! Depcrate_cowimpl_20 {
() => {
// Module: crate::cow
// Provides: {"impl_20"}
// Dependencies: {}
impl RawVarZeroCow { # [doc = " Whether or not this is owned"] # [inline] pub fn is_owned (& self) -> bool { # [cfg (feature = "alloc")] return self . owned ; # [cfg (not (feature = "alloc"))] return false ; } # [doc = " Get the byte representation of this type"] # [inline] pub fn as_bytes (& self) -> & [u8] { unsafe { self . buf . as_ref () } } }
};
}
