macro_rules! deps {
    () => {
        TokenTree!();
        Subtree!();
        Leaf!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < S : Copy > TokenTree < S > { pub fn first_span (& self) -> S { match self { TokenTree :: Leaf (l) => * l . span () , TokenTree :: Subtree (s) => s . delimiter . open , } } }
    };
}

impl_11!()