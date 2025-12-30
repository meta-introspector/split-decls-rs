// Generated macro for encode_fractional_secs (function)
macro_rules! Depcrate_generalized_time_nanosencode_fractional_secs {
() => {
// Module: crate::generalized_time_nanos
// Provides: {"encode_fractional_secs"}
// Dependencies: {}
# [doc = " Encodes the given nanoseconds as fractional secs, discarding trailing"] # [doc = " zeroes."] fn encode_fractional_secs < W > (writer : & mut W , tag : Tag , nanoseconds : u32) -> Result < () > where W : Writer + ? Sized , { if nanoseconds >= 1_000_000_000 { return Err (tag . value_error () . into ()) ; } for_each_digits_without_trailing_zeroes (nanoseconds , | cur_val | { writer . write_byte (b'0' . checked_add (cur_val) . ok_or (ErrorKind :: Overflow) ?) }) ? ; Ok (()) }
};
}
