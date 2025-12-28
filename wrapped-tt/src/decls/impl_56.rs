macro_rules! deps {
    () => {
        Literal!();
        Leaf!();
        Punct!();
        Ident!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < S > fmt :: Display for Leaf < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Leaf :: Ident (it) => fmt :: Display :: fmt (it , f) , Leaf :: Literal (it) => fmt :: Display :: fmt (it , f) , Leaf :: Punct (it) => fmt :: Display :: fmt (it , f) , } } }
    };
}

impl_56!();