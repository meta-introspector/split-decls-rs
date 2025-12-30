// Generated macro for Type (struct)
macro_rules! DepcrateType {
() => {
// Module: crate
// Provides: {"Type"}
// Dependencies: {}
# [doc = " Specification of communication semantics on a socket."] # [doc = ""] # [doc = " This is a newtype wrapper around an integer which provides a nicer API in"] # [doc = " addition to an injection point for documentation. Convenience constructors"] # [doc = " such as `Type::stream`, `Type::dgram`, etc, are provided to avoid reaching"] # [doc = " into libc for various constants."] # [doc = ""] # [doc = " This type is freely interconvertible with the `i32` type, however, if a raw"] # [doc = " value needs to be provided."] pub struct Type (i32) ;
};
}
