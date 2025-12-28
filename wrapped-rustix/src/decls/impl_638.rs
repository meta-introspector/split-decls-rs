macro_rules! deps {
    () => {
        Result!();
        SocketAddrAny!();
    };
}

macro_rules! impl_638 {
    () => {
        deps!();
        # [cfg (unix)] impl TryFrom < SocketAddrAny > for SocketAddrUnix { type Error = Errno ; # [doc = " Convert if the address is a Unix socket address."] # [doc = ""] # [doc = " Returns `Err(Errno::AFNOSUPPORT)` if the address family is not Unix."] # [inline] fn try_from (value : SocketAddrAny) -> Result < Self , Self :: Error > { read_sockaddr :: read_sockaddr_unix (& value) } }
    };
}

impl_638!();