macro_rules! deps {
    () => {
        SocketAddrAny!();
        Result!();
    };
}

macro_rules! impl_634 {
    () => {
        deps!();
        impl TryFrom < SocketAddrAny > for SocketAddrV4 { type Error = Errno ; # [doc = " Convert if the address is an IPv4 address."] # [doc = ""] # [doc = " Returns `Err(Errno::AFNOSUPPORT)` if the address family is not IPv4."] # [inline] fn try_from (value : SocketAddrAny) -> Result < Self , Self :: Error > { read_sockaddr :: read_sockaddr_v4 (& value) } }
    };
}

impl_634!();