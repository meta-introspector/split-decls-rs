// Generated macro for TryFromParsed (enum)
macro_rules! Depcrate_error_try_from_parsedTryFromParsed {
() => {
// Module: crate::error::try_from_parsed
// Provides: {"TryFromParsed"}
// Dependencies: {}
# [doc = " An error that occurred when converting a [`Parsed`](crate::parsing::Parsed) to another type."] # [non_exhaustive] # [allow (variant_size_differences , reason = "only triggers on some platforms")] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum TryFromParsed { # [doc = " The [`Parsed`](crate::parsing::Parsed) did not include enough information to construct the"] # [doc = " type."] InsufficientInformation , # [doc = " Some component contained an invalid value for the type."] ComponentRange (error :: ComponentRange) , }
};
}
