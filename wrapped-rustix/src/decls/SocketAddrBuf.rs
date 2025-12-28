macro_rules! deps {
    () => {
        SocketAddrAny!();
    };
}

macro_rules! SocketAddrBuf {
    () => {
        deps!();
        # [doc = " Temporary buffer for creating a `SocketAddrAny` from a syscall that writes"] # [doc = " to a `sockaddr_t` and `socklen_t`"] # [doc = ""] # [doc = " Unlike `SocketAddrAny`, this does not maintain the invariant that `len`"] # [doc = " bytes are initialized."] pub (crate) struct SocketAddrBuf { pub (crate) len : c :: socklen_t , pub (crate) storage : MaybeUninit < SocketAddrStorage > , }
    };
}

SocketAddrBuf!()