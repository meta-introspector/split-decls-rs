use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Type {
    /// Type corresponding to `SOCK_STREAM`.
    ///
    /// Used for protocols such as TCP.
    pub const STREAM: Type = Type(sys::SOCK_STREAM);
    /// Type corresponding to `SOCK_DGRAM`.
    ///
    /// Used for protocols such as UDP.
    pub const DGRAM: Type = Type(sys::SOCK_DGRAM);
    /// Type corresponding to `SOCK_DCCP`.
    ///
    /// Used for the DCCP protocol.
    #[cfg(all(feature = "all", target_os = "linux"))]
    pub const DCCP: Type = Type(sys::SOCK_DCCP);
    /// Type corresponding to `SOCK_SEQPACKET`.
    #[cfg(all(feature = "all", not(target_os = "espidf")))]
    pub const SEQPACKET: Type = Type(sys::SOCK_SEQPACKET);
    /// Type corresponding to `SOCK_RAW`.
    #[cfg(all(feature = "all", not(any(target_os = "redox", target_os = "espidf"))))]
    pub const RAW: Type = Type(sys::SOCK_RAW);
}
