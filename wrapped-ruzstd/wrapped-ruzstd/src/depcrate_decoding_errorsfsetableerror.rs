// Generated macro for FSETableError (enum)
macro_rules! Depcrate_decoding_errorsFSETableError {
() => {
// Module: crate::decoding::errors
// Provides: {"FSETableError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum FSETableError { AccLogIsZero , AccLogTooBig { got : u8 , max : u8 , } , GetBitsError (GetBitsError) , ProbabilityCounterMismatch { got : u32 , expected_sum : u32 , symbol_probabilities : Vec < i32 > , } , TooManySymbols { got : usize , } , }
};
}
