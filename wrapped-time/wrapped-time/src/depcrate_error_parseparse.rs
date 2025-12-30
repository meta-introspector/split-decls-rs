// Generated macro for Parse (enum)
macro_rules! Depcrate_error_parseParse {
() => {
// Module: crate::error::parse
// Provides: {"Parse"}
// Dependencies: {}
# [doc = " An error that occurred at some stage of parsing."] # [non_exhaustive] # [allow (variant_size_differences , reason = "only triggers on some platforms")] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Parse { # [expect (missing_docs)] TryFromParsed (TryFromParsed) , # [expect (missing_docs)] ParseFromDescription (ParseFromDescription) , # [expect (missing_docs)] # [non_exhaustive] # [deprecated (since = "0.3.28" , note = "no longer output. moved to the `ParseFromDescription` variant")] UnexpectedTrailingCharacters { # [doc (hidden)] never : Infallible , } , }
};
}
