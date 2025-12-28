macro_rules! deps {
    () => {
        Punct!();
        Ident!();
        Leaf!();
        Literal!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < S > Leaf < S > { pub fn span (& self) -> & S { match self { Leaf :: Literal (it) => & it . span , Leaf :: Punct (it) => & it . span , Leaf :: Ident (it) => & it . span , } } }
    };
}

impl_24!()