// Generated macro for text_additions (module)
macro_rules! Depcrate_typestext_additions {
() => {
// Module: crate::types
// Provides: {"text_additions"}
// Dependencies: {}
# [cfg (feature = "text")] mod text_additions { use super :: * ; use crate :: text :: DiffableStr ; use std :: borrow :: Cow ; # [doc = " The text interface can produce changes over [`DiffableStr`] implementing"] # [doc = " values.  As those are generic interfaces for different types of strings"] # [doc = " utility methods to make working with standard rust strings more enjoyable."] impl < 's , T : DiffableStr + ? Sized > Change < & 's T > { # [doc = " Returns the value as string if it is utf-8."] pub fn as_str (& self) -> Option < & 's str > { T :: as_str (self . value) } # [doc = " Returns the value (lossy) decoded as utf-8 string."] pub fn to_string_lossy (& self) -> Cow < 's , str > { T :: to_string_lossy (self . value) } # [doc = " Returns `true` if this change does not end in a newline and must be"] # [doc = " followed up by one if line based diffs are used."] # [doc = ""] # [doc = " The [`std::fmt::Display`] implementation of [`Change`] will automatically"] # [doc = " insert a newline after the value if this is true."] pub fn missing_newline (& self) -> bool { ! T :: ends_with_newline (self . value) } } impl < T : DiffableStr + ? Sized > fmt :: Display for Change < & T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}{}" , self . to_string_lossy () , if self . missing_newline () { "\n" } else { "" }) } } }
};
}
