// Generated macro for DeserializeBytes (trait)
macro_rules! DepcrateDeserializeBytes {
() => {
// Module: crate
// Provides: {"DeserializeBytes"}
// Dependencies: {}
# [doc = " The `DeserializeBytes` trait defines functions to deserialize a byte slice"] # [doc = " to a struct or enum. In contrast to [`Deserialize`], this trait operates"] # [doc = " directly on byte slices and can return any remaining bytes."] pub trait DeserializeBytes : Size { # [doc = " This function deserializes the `bytes` from the provided a `&[u8]`"] # [doc = " and returns the populated struct, as well as the remaining slice."] # [doc = ""] # [doc = " In order to get the amount of bytes read, use [`Size::tls_serialized_len`]."] # [doc = ""] # [doc = " Returns an error if one occurs during deserialization."] fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > where Self : Sized ; # [doc = " This function deserializes the provided `bytes` and returns the populated"] # [doc = " struct. All bytes must be consumed."] # [doc = ""] # [doc = " Returns an error if not all bytes are read from the input, or if an error"] # [doc = " occurs during deserialization."] fn tls_deserialize_exact_bytes (bytes : & [u8]) -> Result < Self , Error > where Self : Sized , { let (out , remainder) = Self :: tls_deserialize_bytes (bytes) ? ; if ! remainder . is_empty () { return Err (Error :: TrailingData) ; } Ok (out) } }
};
}
