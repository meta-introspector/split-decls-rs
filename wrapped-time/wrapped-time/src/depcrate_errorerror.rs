// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " A unified error type for anything returned by a method in the time crate."] # [doc = ""] # [doc = " This can be used when you either don't know or don't care about the exact error returned."] # [doc = " `Result<_, time::Error>` (or its alias `time::Result<_>`) will work in these situations."] # [non_exhaustive] # [derive (Debug)] pub enum Error { # [expect (missing_docs)] ConversionRange (ConversionRange) , # [expect (missing_docs)] ComponentRange (ComponentRange) , # [cfg (feature = "local-offset")] # [expect (missing_docs)] IndeterminateOffset (IndeterminateOffset) , # [cfg (feature = "formatting")] # [expect (missing_docs)] Format (Format) , # [cfg (feature = "parsing")] # [expect (missing_docs)] ParseFromDescription (ParseFromDescription) , # [cfg (feature = "parsing")] # [expect (missing_docs)] # [non_exhaustive] # [deprecated (since = "0.3.28" , note = "no longer output. moved to the `ParseFromDescription` variant")] UnexpectedTrailingCharacters { # [doc (hidden)] never : Infallible , } , # [cfg (feature = "parsing")] # [expect (missing_docs)] TryFromParsed (TryFromParsed) , # [cfg (all (any (feature = "formatting" , feature = "parsing") , feature = "alloc"))] # [expect (missing_docs)] InvalidFormatDescription (InvalidFormatDescription) , # [expect (missing_docs)] DifferentVariant (DifferentVariant) , # [expect (missing_docs)] InvalidVariant (InvalidVariant) , }
};
}
