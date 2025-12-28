macro_rules! deps {
    () => {
        ReusableBoxFuture!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < T > Future for ReusableBoxFuture < '_ , T > { type Output = T ; # [doc = " Poll the future stored inside this box."] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { Pin :: into_inner (self) . get_pin () . poll (cx) } }
    };
}

impl_79!()