macro_rules! deps {
    () => {
        SocketAddrAny!();
    };
}

macro_rules! impl_635 {
    () => {
        deps!();
        impl From < SocketAddrV6 > for SocketAddrAny { # [inline] fn from (from : SocketAddrV6) -> Self { from . as_any () } }
    };
}

impl_635!()