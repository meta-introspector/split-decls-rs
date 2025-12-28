macro_rules! deps {
    () => {
        RawDatabase!();
    };
}

macro_rules! ErasedDatabaseDownCasterSig {
    () => {
        deps!();
        type ErasedDatabaseDownCasterSig = unsafe fn (RawDatabase < '_ >) -> NonNull < () > ;
    };
}

ErasedDatabaseDownCasterSig!();