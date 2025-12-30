// Generated macro for impl_74 (impl)
macro_rules! Depcrate_baseimpl_74 {
() => {
// Module: crate::base
// Provides: {"impl_74"}
// Dependencies: {}
impl Error { # [doc = " Creates a new `Error` from a status code."] # [doc = " The code must not be zero"] # [inline] # [must_use] pub fn from_code (code : OSStatus) -> Self { Self (NonZeroI32 :: new (code) . unwrap_or_else (| | NonZeroI32 :: new (1) . unwrap ())) } # [doc = " Returns a string describing the current error, if available."] # [inline (always)] # [must_use] pub fn message (self) -> Option < String > { self . inner_message () } # [cold] fn inner_message (self) -> Option < String > { use core_foundation :: base :: TCFType ; use security_framework_sys :: base :: SecCopyErrorMessageString ; use std :: ptr ; unsafe { let s = SecCopyErrorMessageString (self . code () , ptr :: null_mut ()) ; if s . is_null () { None } else { Some (CFString :: wrap_under_create_rule (s) . to_string ()) } } } # [doc = " Returns the code of the current error."] # [inline (always)] # [must_use] pub const fn code (self) -> OSStatus { self . 0 . get () as _ } }
};
}
