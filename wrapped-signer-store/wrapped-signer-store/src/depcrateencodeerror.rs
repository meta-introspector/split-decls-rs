// Generated macro for EncodeError (enum)
macro_rules! DepcrateEncodeError {
() => {
// Module: crate
// Provides: {"EncodeError"}
// Dependencies: {}
# [doc = " An error that can occur during the encoding process."] # [derive (Error , Debug , PartialEq , Eq)] pub enum EncodeError { # [error ("in Base3 encoding, the provided bit-vectors have unmatching lengths")] MismatchedLengths , # [error ("in Base3 encoding, the invalid combination `(true, true)` was found")] InvalidBitCombination , # [error ("the length of the input vectors exceeds u16::MAX (65,535)")] LengthExceedsLimit , # [error ("an arithmetic operation resulted in an overflow")] ArithmeticOverflow , }
};
}
