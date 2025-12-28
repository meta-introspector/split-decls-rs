macro_rules! deps {
    () => {
        Result!();
        SocketAddrAny!();
    };
}

macro_rules! impl_636 {
    () => {
        deps!();
        impl TryFrom < SocketAddrAny > for SocketAddrV6 { type Error = Errno ; # [doc = " Convert if the address is an IPv6 address."] # [doc = ""] # [doc = " Returns `Err(Errno::AFNOSUPPORT)` if the address family is not IPv6."] # [inline] fn try_from (value : SocketAddrAny) -> Result < Self , Self :: Error > { read_sockaddr :: read_sockaddr_v6 (& value) } }
    };
}

impl_636!()