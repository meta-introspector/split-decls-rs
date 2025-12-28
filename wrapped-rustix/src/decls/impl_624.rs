macro_rules! deps {
    () => {
        SocketAddrAny!();
    };
}

macro_rules! impl_624 {
    () => {
        deps!();
        impl PartialEq < Self > for SocketAddrAny { fn eq (& self , other : & Self) -> bool { self . bytes () == other . bytes () } }
    };
}

impl_624!();