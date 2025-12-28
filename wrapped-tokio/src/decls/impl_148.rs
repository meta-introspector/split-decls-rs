macro_rules! deps {
    () => {
        ToSocketAddrs!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < T : ToSocketAddrs + ? Sized > ToSocketAddrs for & T { }
    };
}

impl_148!()