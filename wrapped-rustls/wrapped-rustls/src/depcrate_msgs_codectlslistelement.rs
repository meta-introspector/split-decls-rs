// Generated macro for TlsListElement (trait)
macro_rules! Depcrate_msgs_codecTlsListElement {
() => {
// Module: crate::msgs::codec
// Provides: {"TlsListElement"}
// Dependencies: {}
# [doc = " A trait for types that can be encoded and decoded in a list."] # [doc = ""] # [doc = " This trait is used to implement `Codec` for `Vec<T>`. Lists in the TLS wire format are"] # [doc = " prefixed with a length, the size of which depends on the type of the list elements."] # [doc = " As such, the `Codec` implementation for `Vec<T>` requires an implementation of this trait"] # [doc = " for its element type `T`."] pub (crate) trait TlsListElement { const SIZE_LEN : ListLength ; }
};
}
