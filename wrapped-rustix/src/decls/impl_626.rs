macro_rules! deps {
    () => {
        SocketAddrAny!();
    };
}

macro_rules! impl_626 {
    () => {
        deps!();
        # [allow (clippy :: non_canonical_partial_ord_impl)] impl PartialOrd < Self > for SocketAddrAny { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . bytes () . partial_cmp (other . bytes ()) } }
    };
}

impl_626!()