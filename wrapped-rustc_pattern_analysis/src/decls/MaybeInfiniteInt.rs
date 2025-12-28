macro_rules! MaybeInfiniteInt {
    () => {
        # [doc = " A possibly infinite integer. Values are encoded such that the ordering on `u128` matches the"] # [doc = " natural order on the original type. For example, `-128i8` is encoded as `0` and `127i8` as"] # [doc = " `255`. See `signed_bias` for details."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] pub enum MaybeInfiniteInt { NegInfinity , # [doc = " Encoded value. DO NOT CONSTRUCT BY HAND; use `new_finite_{int,uint}`."] # [non_exhaustive] Finite (u128) , PosInfinity , }
    };
}

MaybeInfiniteInt!();