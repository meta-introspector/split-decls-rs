macro_rules! deps {
    () => {
        SubtreeView!();
        TokenTreesView!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < S : fmt :: Debug + Copy > fmt :: Debug for SubtreeView < '_ , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& TokenTreesView (self . 0) , f) } }
    };
}

impl_28!()