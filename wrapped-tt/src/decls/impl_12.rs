macro_rules! deps {
    () => {
        TtElement!();
        Leaf!();
        Subtree!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < S : Copy > TtElement < '_ , S > { # [inline] pub fn first_span (& self) -> S { match self { TtElement :: Leaf (it) => * it . span () , TtElement :: Subtree (it , _) => it . delimiter . open , } } }
    };
}

impl_12!()