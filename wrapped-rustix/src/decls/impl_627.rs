macro_rules! deps {
    () => {
        SocketAddrAny!();
    };
}

macro_rules! impl_627 {
    () => {
        deps!();
        impl Ord for SocketAddrAny { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . bytes () . cmp (other . bytes ()) } }
    };
}

impl_627!()