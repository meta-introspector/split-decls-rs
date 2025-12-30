// Generated macro for macro_246 (macro)
macro_rules! Depcrate_de_implsmacro_246 {
() => {
// Module: crate::de::impls
// Provides: {"macro_246"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] parse_socket_impl ! { net :: SocketAddrV6 , "IPv6 socket address" , | (ip , port) | net :: SocketAddrV6 :: new (ip , port , 0 , 0) , }
};
}
