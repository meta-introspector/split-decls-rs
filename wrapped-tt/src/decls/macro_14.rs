macro_rules! deps {
    () => {
        Ident!();
        Literal!();
        Punct!();
        Leaf!();
    };
}

macro_rules! macro_14 {
    () => {
        deps!();
        impl_from ! (Literal < S >, Punct < S >, Ident < S > for Leaf) ;
    };
}

macro_14!()