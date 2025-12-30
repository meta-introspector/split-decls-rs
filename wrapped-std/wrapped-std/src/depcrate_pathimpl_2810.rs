// Generated macro for impl_2810 (impl)
macro_rules! Depcrate_pathimpl_2810 {
() => {
// Module: crate::path
// Provides: {"impl_2810"}
// Dependencies: {}
impl < 'a > PrefixComponent < 'a > { # [doc = " Returns the parsed prefix data."] # [doc = ""] # [doc = " See [`Prefix`]'s documentation for more information on the different"] # [doc = " kinds of prefixes."] # [stable (feature = "rust1" , since = "1.0.0")] # [must_use] # [inline] pub fn kind (& self) -> Prefix < 'a > { self . parsed } # [doc = " Returns the raw [`OsStr`] slice for this prefix."] # [stable (feature = "rust1" , since = "1.0.0")] # [must_use] # [inline] pub fn as_os_str (& self) -> & 'a OsStr { self . raw } }
};
}
