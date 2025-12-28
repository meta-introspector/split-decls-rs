macro_rules! deps {
    () => {
        Literal!();
        Ident!();
        Punct!();
        Leaf!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < S > Leaf < S > { pub fn span (& self) -> & S { match self { Leaf :: Literal (it) => & it . span , Leaf :: Punct (it) => & it . span , Leaf :: Ident (it) => & it . span , } } }
    };
}

impl_24!();