// Generated macro for ListLength (enum)
macro_rules! Depcrate_msgs_codecListLength {
() => {
// Module: crate::msgs::codec
// Provides: {"ListLength"}
// Dependencies: {}
# [doc = " The length of the length prefix for a list."] # [doc = ""] # [doc = " The types that appear in lists are limited to three kinds of length prefixes:"] # [doc = " 1, 2, and 3 bytes. For the latter kind, we require a `TlsListElement` implementer"] # [doc = " to specify a maximum length and error if the actual length is larger."] pub (crate) enum ListLength { # [doc = " U8 but non-empty"] NonZeroU8 { empty_error : InvalidMessage } , # [doc = " U16, perhaps empty"] U16 , # [doc = " U16 but non-empty"] NonZeroU16 { empty_error : InvalidMessage } , # [doc = " U24 with imposed upper bound"] U24 { max : usize , error : InvalidMessage } , # [doc = " U24 but non-empty, with imposed upper bound"] NonZeroU24 { max : usize , empty_error : InvalidMessage , too_many_error : InvalidMessage , } , }
};
}
