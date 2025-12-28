macro_rules! deps {
    () => {
        Leaf!();
        Subtree!();
        TtElement!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < S : Copy + fmt :: Debug > fmt :: Debug for TtElement < '_ , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Leaf (leaf) => f . debug_tuple ("Leaf") . field (leaf) . finish () , Self :: Subtree (subtree , inner) => { f . debug_tuple ("Subtree") . field (subtree) . field (inner) . finish () } } } }
    };
}

impl_11!()