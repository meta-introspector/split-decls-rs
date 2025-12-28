macro_rules! deps {
    () => {
        SocketAddrAny!();
    };
}

macro_rules! impl_631 {
    () => {
        deps!();
        impl From < SocketAddr > for SocketAddrAny { # [inline] fn from (from : SocketAddr) -> Self { from . as_any () } }
    };
}

impl_631!();