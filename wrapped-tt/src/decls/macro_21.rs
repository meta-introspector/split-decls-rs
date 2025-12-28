macro_rules! deps {
    () => {
        TokenTree!();
        Leaf!();
        Subtree!();
    };
}

macro_rules! macro_21 {
    () => {
        deps!();
        impl_from ! (Leaf < S >, Subtree < S > for TokenTree) ;
    };
}

macro_21!();