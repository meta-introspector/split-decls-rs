macro_rules! deps {
    () => {
        RawDatabase!();
    };
}

macro_rules! DatabaseDownCasterSig {
    () => {
        deps!();
        type DatabaseDownCasterSig < DbView > = unsafe fn (RawDatabase < '_ >) -> NonNull < DbView > ;
    };
}

DatabaseDownCasterSig!();