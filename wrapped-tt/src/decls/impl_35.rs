macro_rules! deps {
    () => {
        TokenTreesView!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < S : fmt :: Debug + Copy > fmt :: Debug for TokenTreesView < '_ , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut iter = self . iter () ; while let Some (tt) = iter . next () { print_debug_token (f , 0 , tt) ? ; if ! iter . is_empty () { writeln ! (f) ? ; } } Ok (()) } }
    };
}

impl_35!();