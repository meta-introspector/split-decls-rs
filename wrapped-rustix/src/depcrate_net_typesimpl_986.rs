// Generated macro for impl_986 (impl)
macro_rules! Depcrate_net_typesimpl_986 {
() => {
// Module: crate::net::types
// Provides: {"impl_986"}
// Dependencies: {}
# [rustfmt :: skip] impl SocketType { # [doc = " `SOCK_STREAM`"] pub const STREAM : Self = Self (c :: SOCK_STREAM as _) ; # [doc = " `SOCK_DGRAM`"] pub const DGRAM : Self = Self (c :: SOCK_DGRAM as _) ; # [doc = " `SOCK_SEQPACKET`"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon")))] pub const SEQPACKET : Self = Self (c :: SOCK_SEQPACKET as _) ; # [doc = " `SOCK_RAW`"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon")))] pub const RAW : Self = Self (c :: SOCK_RAW as _) ; # [doc = " `SOCK_RDM`"] # [cfg (not (any (target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "redox")))] pub const RDM : Self = Self (c :: SOCK_RDM as _) ; # [doc = " Constructs a `SocketType` from a raw integer."] # [inline] pub const fn from_raw (raw : RawSocketType) -> Self { Self (raw) } # [doc = " Returns the raw integer for this `SocketType`."] # [inline] pub const fn as_raw (self) -> RawSocketType { self . 0 } }
};
}
