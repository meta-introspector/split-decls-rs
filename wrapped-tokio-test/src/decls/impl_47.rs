macro_rules! deps {
    () => {
        Spawn!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < T : Future > Future for Spawn < T > { type Output = T :: Output ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . future . as_mut () . poll (cx) } }
    };
}

impl_47!();