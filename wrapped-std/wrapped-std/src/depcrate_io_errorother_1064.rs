// Generated macro for other_1064 (other)
macro_rules! Depcrate_io_errorother_1064 {
() => {
// Module: crate::io::error
// Provides: {"other_1064"}
// Dependencies: {}
# [doc = " Creates a new I/O error from a known kind of error and a string literal."] # [doc = ""] # [doc = " Contrary to [`Error::new`], this macro does not allocate and can be used in"] # [doc = " `const` contexts."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " #![feature(io_const_error)]"] # [doc = " use std::io::{const_error, Error, ErrorKind};"] # [doc = ""] # [doc = " const FAIL: Error = const_error!(ErrorKind::Unsupported, \"tried something that never works\");"] # [doc = ""] # [doc = " fn not_here() -> Result<(), Error> {"] # [doc = "     Err(FAIL)"] # [doc = " }"] # [doc = " ```"] # [rustc_macro_transparency = "semitransparent"] # [unstable (feature = "io_const_error" , issue = "133448")] # [allow_internal_unstable (hint_must_use , io_const_error_internals)] pub macro const_error ($ kind : expr , $ message : expr $ (,) ?) { $ crate :: hint :: must_use ($ crate :: io :: Error :: from_static_message (const { &$ crate :: io :: SimpleMessage { kind : $ kind , message : $ message } } ,)) }
};
}
