// Generated macro for DecodeError (enum)
macro_rules! DepcrateDecodeError {
() => {
// Module: crate
// Provides: {"DecodeError"}
// Dependencies: {}
# [doc = " An error that can occur during the decoding process."] # [derive (Error , Debug , PartialEq , Eq)] pub enum DecodeError { # [error ("the input slice is too short to be valid")] InputTooShort , # [error ("the encoding version byte is unsupported")] UnsupportedEncoding , # [error ("the data payload is not of the expected length")] CorruptDataPayload , # [error ("an arithmetic operation resulted in an overflow")] ArithmeticOverflow , }
};
}
