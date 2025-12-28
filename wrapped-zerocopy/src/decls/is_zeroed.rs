macro_rules! deps {
    () => {
        Validity!();
        Initialized!();
        KnownLayout!();
        Invariants!();
        Aliasing!();
        Immutable!();
        Reference!();
        BecauseImmutable!();
    };
}

macro_rules! is_zeroed {
    () => {
        deps!();
        # [doc = " Checks if the referent is zeroed."] pub (crate) fn is_zeroed < T , I > (ptr : Ptr < '_ , T , I >) -> bool where T : crate :: Immutable + crate :: KnownLayout , I : invariant :: Invariants < Validity = invariant :: Initialized > , I :: Aliasing : invariant :: Reference , { ptr . as_bytes :: < BecauseImmutable > () . as_ref () . iter () . all (| & byte | byte == 0) }
    };
}

is_zeroed!()