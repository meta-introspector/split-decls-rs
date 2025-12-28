macro_rules! deps {
    () => {
        InternerInner!();
    };
}

macro_rules! Interner {
    () => {
        deps!();
        pub (crate) struct Interner (Lock < InternerInner >) ;
    };
}

Interner!()