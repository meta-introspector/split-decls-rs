// Generated macro for other_error (module)
macro_rules! Depcrate_errorother_error {
() => {
// Module: crate::error
// Provides: {"other_error"}
// Dependencies: {}
mod other_error { use core :: error :: Error as StdError ; use core :: fmt ; use super :: Error ; use crate :: sync :: Arc ; # [doc = " Any other error that cannot be expressed by a more specific [`Error`] variant."] # [doc = ""] # [doc = " For example, an `OtherError` could be produced by a custom crypto provider"] # [doc = " exposing a provider specific error."] # [doc = ""] # [doc = " Enums holding this type will never compare equal to each other."] # [derive (Debug , Clone)] pub struct OtherError (Arc < dyn StdError + Send + Sync >) ; impl OtherError { # [doc = " Create a new `OtherError` from any error type."] pub fn new (err : impl StdError + Send + Sync + 'static) -> Self { Self (Arc :: new (err)) } } impl PartialEq < Self > for OtherError { fn eq (& self , _other : & Self) -> bool { false } } impl From < OtherError > for Error { fn from (value : OtherError) -> Self { Self :: Other (value) } } impl fmt :: Display for OtherError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (feature = "std")] { write ! (f , "{}" , self . 0) } # [cfg (not (feature = "std"))] { f . write_str ("no further information available") } } } impl StdError for OtherError { fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (self . 0 . as_ref ()) } } }
};
}
