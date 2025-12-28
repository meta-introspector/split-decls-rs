macro_rules! deps {
    () => {
        Leaf!();
        Literal!();
        Ident!();
        Punct!();
    };
}

macro_rules! macro_25 {
    () => {
        deps!();
        impl_from ! (Literal < S >, Punct < S >, Ident < S > for Leaf) ;
    };
}

macro_25!();