// Generated macro for ErrorConvert (trait)
macro_rules! Depcrate_errorErrorConvert {
() => {
// Module: crate::error
// Provides: {"ErrorConvert"}
// Dependencies: {}
# [doc = " Equivalent of `From` implementation to avoid orphan rules in bits parsers"] pub trait ErrorConvert < E > { # [doc = " Transform to another error type"] fn convert (self) -> E ; }
};
}
