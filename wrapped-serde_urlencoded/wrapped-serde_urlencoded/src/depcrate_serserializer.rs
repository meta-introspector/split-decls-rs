// Generated macro for Serializer (struct)
macro_rules! Depcrate_serSerializer {
() => {
// Module: crate::ser
// Provides: {"Serializer"}
// Dependencies: {}
# [doc = " A serializer for the `application/x-www-form-urlencoded` format."] # [doc = ""] # [doc = " * Supported top-level inputs are structs, maps and sequences of pairs,"] # [doc = "   with or without a given length."] # [doc = ""] # [doc = " * Supported keys and values are integers, bytes (if convertible to strings),"] # [doc = "   unit structs and unit variants."] # [doc = ""] # [doc = " * Newtype structs defer to their inner values."] pub struct Serializer < 'input , 'output , Target : UrlEncodedTarget > { urlencoder : & 'output mut UrlEncodedSerializer < 'input , Target > , }
};
}
