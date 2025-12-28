macro_rules! deps {
    () => {
        Bound!();
        Version!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl PartialOrd < Bound > for Version { fn partial_cmp (& self , rhs : & Bound) -> Option < Ordering > { match rhs { Bound :: Nightly (date) => match self . channel { Stable | Beta => Some (Ordering :: Less) , Nightly (nightly) => Some (nightly . cmp (date)) , Dev => Some (Ordering :: Greater) , } , Bound :: Stable (release) => { let version = (self . minor , self . patch) ; let bound = (release . minor , release . patch . unwrap_or (0)) ; Some (version . cmp (& bound)) } } } }
    };
}

impl_8!();