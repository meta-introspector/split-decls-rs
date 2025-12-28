macro_rules! SocketAddrLen {
    () => {
        # [doc = " A type for the length of a socket address."] # [doc = ""] # [doc = " This type will always be big enough to hold any socket address, but never"] # [doc = " bigger than `usize`."] # [doc (alias = "socklen_t")] pub type SocketAddrLen = u32 ;
    };
}

SocketAddrLen!()