// Generated macro for Domain (struct)
macro_rules! DepcrateDomain {
() => {
// Module: crate
// Provides: {"Domain"}
// Dependencies: {}
# [doc = " Specification of the communication domain for a socket."] # [doc = ""] # [doc = " This is a newtype wrapper around an integer which provides a nicer API in"] # [doc = " addition to an injection point for documentation. Convenience constructors"] # [doc = " such as `Domain::ipv4`, `Domain::ipv6`, etc, are provided to avoid reaching"] # [doc = " into libc for various constants."] # [doc = ""] # [doc = " This type is freely interconvertible with the `i32` type, however, if a raw"] # [doc = " value needs to be provided."] pub struct Domain (i32) ;
};
}
