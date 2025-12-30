// Generated macro for Deserialize (trait)
macro_rules! DepcrateDeserialize {
() => {
// Module: crate
// Provides: {"Deserialize"}
// Dependencies: {}
# [doc = " The `Deserialize` trait defines functions to deserialize a byte slice to a"] # [doc = " struct or enum."] pub trait Deserialize : Size { # [doc = " This function deserializes the `bytes` from the provided a [`std::io::Read`]"] # [doc = " and returns the populated struct."] # [doc = ""] # [doc = " In order to get the amount of bytes read, use [`Size::tls_serialized_len`]."] # [doc = ""] # [doc = " Returns an error if one occurs during deserialization."] # [cfg (feature = "std")] fn tls_deserialize < R : Read > (bytes : & mut R) -> Result < Self , Error > where Self : Sized ; # [doc = " This function deserializes the provided `bytes` and returns the populated"] # [doc = " struct. All bytes must be consumed."] # [doc = ""] # [doc = " Returns an error if not all bytes are read from the input, or if an error"] # [doc = " occurs during deserialization."] # [cfg (feature = "std")] fn tls_deserialize_exact (bytes : impl AsRef < [u8] >) -> Result < Self , Error > where Self : Sized , { let mut bytes = bytes . as_ref () ; let out = Self :: tls_deserialize (& mut bytes) ? ; if ! bytes . is_empty () { return Err (Error :: TrailingData) ; } Ok (out) } }
};
}
