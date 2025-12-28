macro_rules! deps {
    () => {
        TokenTreesView!();
        SubtreeView!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < S : Copy > fmt :: Display for SubtreeView < '_ , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& TokenTreesView (self . 0) , f) } }
    };
}

impl_40!();