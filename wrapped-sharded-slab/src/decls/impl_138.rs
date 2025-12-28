macro_rules! deps {
    () => {
        Addr!();
        Config!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < C : cfg :: Config > PartialEq for Addr < C > { fn eq (& self , other : & Self) -> bool { self . addr == other . addr } }
    };
}

impl_138!()