macro_rules! deps {
    () => {
        Initialized!();
        Shared!();
        Aliasing!();
        Unaligned!();
        Alignment!();
    };
}

macro_rules! Maybe {
    () => {
        deps!();
        # [doc = " A shorthand for a maybe-valid, maybe-aligned reference. Used as the argument"] # [doc = " to [`TryFromBytes::is_bit_valid`]."] # [doc = ""] # [doc = " [`TryFromBytes::is_bit_valid`]: crate::TryFromBytes::is_bit_valid"] pub type Maybe < 'a , T , Aliasing = invariant :: Shared , Alignment = invariant :: Unaligned > = Ptr < 'a , T , (Aliasing , Alignment , invariant :: Initialized) > ;
    };
}

Maybe!()