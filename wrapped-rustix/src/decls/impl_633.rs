macro_rules! deps {
    () => {
        SocketAddrAny!();
    };
}

macro_rules! impl_633 {
    () => {
        deps!();
        impl From < SocketAddrV4 > for SocketAddrAny { # [inline] fn from (from : SocketAddrV4) -> Self { from . as_any () } }
    };
}

impl_633!();