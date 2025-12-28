macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl Type { # [doc = " Type corresponding to `SOCK_STREAM`."] # [doc = ""] # [doc = " Used for protocols such as TCP."] pub const STREAM : Type = Type (sys :: SOCK_STREAM) ; # [doc = " Type corresponding to `SOCK_DGRAM`."] # [doc = ""] # [doc = " Used for protocols such as UDP."] pub const DGRAM : Type = Type (sys :: SOCK_DGRAM) ; # [doc = " Type corresponding to `SOCK_DCCP`."] # [doc = ""] # [doc = " Used for the DCCP protocol."] # [cfg (all (feature = "all" , target_os = "linux"))] pub const DCCP : Type = Type (sys :: SOCK_DCCP) ; # [doc = " Type corresponding to `SOCK_SEQPACKET`."] # [cfg (all (feature = "all" , not (target_os = "espidf")))] pub const SEQPACKET : Type = Type (sys :: SOCK_SEQPACKET) ; # [doc = " Type corresponding to `SOCK_RAW`."] # [cfg (all (feature = "all" , not (any (target_os = "redox" , target_os = "espidf"))))] pub const RAW : Type = Type (sys :: SOCK_RAW) ; }
    };
}

impl_55!()