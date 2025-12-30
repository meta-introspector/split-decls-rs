// Generated macro for Format (enum)
macro_rules! Depcrate_error_formatFormat {
() => {
// Module: crate::error::format
// Provides: {"Format"}
// Dependencies: {}
# [doc = " An error occurred when formatting."] # [non_exhaustive] # [derive (Debug)] pub enum Format { # [doc = " The type being formatted does not contain sufficient information to format a component."] # [non_exhaustive] InsufficientTypeInformation , # [doc = " The component named has a value that cannot be formatted into the requested format."] # [doc = ""] # [doc = " This variant is only returned when using well-known formats."] InvalidComponent (& 'static str) , # [doc = " A component provided was out of range."] ComponentRange (Box < error :: ComponentRange >) , # [doc = " A value of `std::io::Error` was returned internally."] StdIo (io :: Error) , }
};
}
