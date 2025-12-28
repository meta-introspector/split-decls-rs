macro_rules! deps {
    () => {
        Leaf!();
        TokenTree!();
        Subtree!();
    };
}

macro_rules! macro_10 {
    () => {
        deps!();
        impl_from ! (Leaf < S >, Subtree < S > for TokenTree) ;
    };
}

macro_10!()