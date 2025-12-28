macro_rules! deps {
    () => {
        Literal!();
        Ident!();
        Leaf!();
        Punct!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < S > Leaf < S > { pub fn span (& self) -> & S { match self { Leaf :: Literal (it) => & it . span , Leaf :: Punct (it) => & it . span , Leaf :: Ident (it) => & it . span , } } }
    };
}

impl_13!()