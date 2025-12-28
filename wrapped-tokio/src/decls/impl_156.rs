macro_rules! deps {
    () => {
        ToSocketAddrs!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl ToSocketAddrs for (IpAddr , u16) { }
    };
}

impl_156!();