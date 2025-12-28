macro_rules! deps {
    () => {
        MaybeInfiniteInt!();
    };
}

macro_rules! IntRange {
    () => {
        deps!();
        # [doc = " An exclusive interval, used for precise integer exhaustiveness checking. `IntRange`s always"] # [doc = " store a contiguous range."] # [doc = ""] # [doc = " `IntRange` is never used to encode an empty range or a \"range\" that wraps around the (offset)"] # [doc = " space: i.e., `range.lo < range.hi`."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct IntRange { pub lo : MaybeInfiniteInt , pub hi : MaybeInfiniteInt , }
    };
}

IntRange!()