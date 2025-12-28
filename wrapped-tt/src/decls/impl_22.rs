macro_rules! deps {
    () => {
        Subtree!();
        Leaf!();
        TokenTree!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < S : Copy > TokenTree < S > { pub fn first_span (& self) -> S { match self { TokenTree :: Leaf (l) => * l . span () , TokenTree :: Subtree (s) => s . delimiter . open , } } }
    };
}

impl_22!()