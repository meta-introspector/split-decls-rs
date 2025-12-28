macro_rules! deps {
    () => {
        SockAddr!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl From < SocketAddr > for SockAddr { fn from (addr : SocketAddr) -> SockAddr { match addr { SocketAddr :: V4 (addr) => addr . into () , SocketAddr :: V6 (addr) => addr . into () , } } }
    };
}

impl_10!()