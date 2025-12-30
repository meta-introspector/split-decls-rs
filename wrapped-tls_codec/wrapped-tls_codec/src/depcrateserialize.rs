// Generated macro for Serialize (trait)
macro_rules! DepcrateSerialize {
() => {
// Module: crate
// Provides: {"Serialize"}
// Dependencies: {}
# [doc = " The `Serialize` trait provides functions to serialize a struct or enum."] # [doc = ""] # [doc = " The trait provides two functions:"] # [doc = " * `tls_serialize` that takes a buffer to write the serialization to"] # [doc = " * `tls_serialize_detached` that returns a byte vector"] pub trait Serialize : Size { # [doc = " Serialize `self` and write it to the `writer`."] # [doc = " The function returns the number of bytes written to `writer`."] # [cfg (feature = "std")] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > ; # [doc = " Serialize `self` and return it as a byte vector."] # [cfg (feature = "std")] fn tls_serialize_detached (& self) -> Result < Vec < u8 > , Error > { let mut buffer = Vec :: with_capacity (self . tls_serialized_len ()) ; let written = self . tls_serialize (& mut buffer) ? ; debug_assert_eq ! (written , buffer . len () , "Expected that {} bytes were written but the output holds {} bytes" , written , buffer . len ()) ; if written != buffer . len () { Err (Error :: EncodingError (format ! ("Expected that {} bytes were written but the output holds {} bytes" , written , buffer . len ()))) } else { Ok (buffer) } } }
};
}
