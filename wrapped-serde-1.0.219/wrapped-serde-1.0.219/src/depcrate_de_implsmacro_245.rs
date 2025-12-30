// Generated macro for macro_245 (macro)
macro_rules! Depcrate_de_implsmacro_245 {
() => {
// Module: crate::de::impls
// Provides: {"macro_245"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] parse_socket_impl ! { net :: SocketAddrV4 , "IPv4 socket address" , | (ip , port) | net :: SocketAddrV4 :: new (ip , port) , }
};
}
