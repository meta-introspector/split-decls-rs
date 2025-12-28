macro_rules! deps {
    () => {
        RawUid!();
        Uid!();
    };
}

macro_rules! impl_1701 {
    () => {
        deps!();
        impl Uid { # [doc = " A `Uid` corresponding to the root user (uid 0)."] pub const ROOT : Self = Self (0) ; # [doc = " Converts a `RawUid` into a `Uid`."] # [doc = ""] # [doc = " `raw` must be the value of a valid Unix user ID, and not `-1`."] # [inline] pub fn from_raw (raw : RawUid) -> Self { debug_assert_ne ! (raw , ! 0) ; Self (raw) } # [doc = " Converts a `RawUid` into a `Uid`."] # [doc = ""] # [doc = " `raw` must be the value of a valid Unix user ID, and not `-1`."] # [inline] pub const fn from_raw_unchecked (raw : RawUid) -> Self { Self (raw) } # [doc = " Converts a `Uid` into a `RawUid`."] # [inline] pub const fn as_raw (self) -> RawUid { self . 0 } # [doc = " Test whether this uid represents the root user ([`Uid::ROOT`])."] # [inline] pub const fn is_root (self) -> bool { self . 0 == Self :: ROOT . 0 } }
    };
}

impl_1701!()