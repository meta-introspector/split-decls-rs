// Generated macro for impl_2502 (impl)
macro_rules! Depcrate_ugidimpl_2502 {
() => {
// Module: crate::ugid
// Provides: {"impl_2502"}
// Dependencies: {}
impl Gid { # [doc = " A `Gid` corresponding to the root group (gid 0)."] pub const ROOT : Self = Self (0) ; # [doc = " Converts a `RawGid` into a `Gid`."] # [doc = ""] # [doc = " `raw` must be the value of a valid Unix group ID, and not `-1`."] # [inline] pub fn from_raw (raw : RawGid) -> Self { debug_assert_ne ! (raw , ! 0) ; Self (raw) } # [doc = " Converts a `RawGid` into a `Gid`."] # [doc = ""] # [doc = " `raw` must be the value of a valid Unix group ID, and not `-1`."] # [inline] pub const fn from_raw_unchecked (raw : RawGid) -> Self { Self (raw) } # [doc = " Converts a `Gid` into a `RawGid`."] # [inline] pub const fn as_raw (self) -> RawGid { self . 0 } # [doc = " Test whether this gid represents the root group ([`Gid::ROOT`])."] # [inline] pub const fn is_root (self) -> bool { self . 0 == Self :: ROOT . 0 } }
};
}
