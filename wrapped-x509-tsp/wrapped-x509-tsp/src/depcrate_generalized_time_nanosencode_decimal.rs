// Generated macro for encode_decimal (function)
macro_rules! Depcrate_generalized_time_nanosencode_decimal {
() => {
// Module: crate::generalized_time_nanos
// Provides: {"encode_decimal"}
// Dependencies: {}
# [doc = " Encode 2-digit decimal value"] fn encode_decimal < W > (writer : & mut W , tag : Tag , value : u8) -> Result < () > where W : Writer + ? Sized , { let hi_val = value / 10 ; if hi_val >= 10 { return Err (tag . value_error () . into ()) ; } writer . write_byte (b'0' . checked_add (hi_val) . ok_or (ErrorKind :: Overflow) ?) ? ; writer . write_byte (b'0' . checked_add (value % 10) . ok_or (ErrorKind :: Overflow) ?) }
};
}
