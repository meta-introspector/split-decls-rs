// Generated macro for impl_56 (impl)
macro_rules! Depcrate_socketimpl_56 {
() => {
// Module: crate::socket
// Provides: {"impl_56"}
// Dependencies: {}
impl Type { # [doc = " Type corresponding to `SOCK_STREAM`"] # [doc = ""] # [doc = " Used for protocols such as TCP."] pub fn stream () -> Type { Type (c :: SOCK_STREAM) } # [doc = " Type corresponding to `SOCK_DGRAM`"] # [doc = ""] # [doc = " Used for protocols such as UDP."] pub fn dgram () -> Type { Type (c :: SOCK_DGRAM) } # [doc = " Type corresponding to `SOCK_SEQPACKET`"] pub fn seqpacket () -> Type { Type (c :: SOCK_SEQPACKET) } # [doc = " Type corresponding to `SOCK_RAW`"] pub fn raw () -> Type { Type (c :: SOCK_RAW) } }
};
}
