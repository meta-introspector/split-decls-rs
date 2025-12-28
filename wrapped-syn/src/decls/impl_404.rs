macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        impl Ord for Lifetime { fn cmp (& self , other : & Lifetime) -> Ordering { self . ident . cmp (& other . ident) } }
    };
}

impl_404!();