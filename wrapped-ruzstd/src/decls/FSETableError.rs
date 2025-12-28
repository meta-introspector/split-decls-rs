macro_rules! deps {
    () => {
        GetBitsError!();
    };
}

macro_rules! FSETableError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum FSETableError { AccLogIsZero , AccLogTooBig { got : u8 , max : u8 , } , GetBitsError (GetBitsError) , ProbabilityCounterMismatch { got : u32 , expected_sum : u32 , symbol_probabilities : Vec < i32 > , } , TooManySymbols { got : usize , } , }
    };
}

FSETableError!();