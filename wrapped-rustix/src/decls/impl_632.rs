macro_rules! deps {
    () => {
        SocketAddrAny!();
        AddressFamily!();
        Result!();
    };
}

macro_rules! impl_632 {
    () => {
        deps!();
        impl TryFrom < SocketAddrAny > for SocketAddr { type Error = Errno ; # [doc = " Convert if the address is an IPv4 or IPv6 address."] # [doc = ""] # [doc = " Returns `Err(Errno::AFNOSUPPORT)` if the address family is not IPv4 or"] # [doc = " IPv6."] # [inline] fn try_from (value : SocketAddrAny) -> Result < Self , Self :: Error > { match value . address_family () { AddressFamily :: INET => read_sockaddr :: read_sockaddr_v4 (& value) . map (SocketAddr :: V4) , AddressFamily :: INET6 => read_sockaddr :: read_sockaddr_v6 (& value) . map (SocketAddr :: V6) , _ => Err (Errno :: AFNOSUPPORT) , } } }
    };
}

impl_632!();