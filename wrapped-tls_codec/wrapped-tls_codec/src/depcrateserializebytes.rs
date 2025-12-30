// Generated macro for SerializeBytes (trait)
macro_rules! DepcrateSerializeBytes {
() => {
// Module: crate
// Provides: {"SerializeBytes"}
// Dependencies: {}
# [doc = " The `SerializeBytes` trait provides a function to serialize a struct or enum."] # [doc = ""] # [doc = " The trait provides one function:"] # [doc = " * `tls_serialize` that returns a byte vector"] pub trait SerializeBytes : Size { # [doc = " Serialize `self` and return it as a byte vector."] fn tls_serialize (& self) -> Result < Vec < u8 > , Error > ; }
};
}
