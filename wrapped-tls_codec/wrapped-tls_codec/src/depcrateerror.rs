// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Errors that are thrown by this crate."] # [derive (Debug , Eq , PartialEq , Clone)] pub enum Error { # [doc = " An error occurred during encoding."] EncodingError (String) , # [doc = " The length of a vector is invalid."] InvalidVectorLength , # [doc = " Error writing everything out."] InvalidWriteLength (String) , # [doc = " Invalid input when trying to decode a primitive integer."] InvalidInput , # [doc = " An error occurred during decoding."] DecodingError (String) , # [doc = " Reached the end of a byte stream."] EndOfStream , # [doc = " Found unexpected data after deserializing."] TrailingData , # [doc = " An unknown value in an enum."] # [doc = " The application might not want to treat this as an error because it is"] # [doc = " only an unknown value, not an invalid value."] UnknownValue (u64) , # [doc = " An internal library error that indicates a bug."] LibraryError , }
};
}
