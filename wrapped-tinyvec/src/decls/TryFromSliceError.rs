macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! TryFromSliceError {
    () => {
        deps!();
        # [doc = " The error type returned when a conversion from a slice to an [`ArrayVec`]"] # [doc = " fails."] # [derive (Debug , Copy , Clone)] pub struct TryFromSliceError (()) ;
    };
}

TryFromSliceError!();