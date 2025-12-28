macro_rules! deps {
    () => {
        SubtreeView!();
        TokenTreesView!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < S : Copy > fmt :: Display for SubtreeView < '_ , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& TokenTreesView (self . 0) , f) } }
    };
}

impl_29!()