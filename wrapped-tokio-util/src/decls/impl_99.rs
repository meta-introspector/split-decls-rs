macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < L , R , O > Future for Either < L , R > where L : Future < Output = O > , R : Future < Output = O > , { type Output = O ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { delegate_call ! (self . poll (cx)) } }
    };
}

impl_99!();