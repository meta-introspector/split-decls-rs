macro_rules! deps {
    () => {
        SocketAddrAny!();
    };
}

macro_rules! impl_637 {
    () => {
        deps!();
        # [cfg (unix)] impl From < SocketAddrUnix > for SocketAddrAny { # [inline] fn from (from : SocketAddrUnix) -> Self { from . as_any () } }
    };
}

impl_637!();