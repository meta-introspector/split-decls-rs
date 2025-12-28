macro_rules! deps {
    () => {
        DropShimElaborator!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl fmt :: Debug for DropShimElaborator < '_ , '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . debug_struct ("DropShimElaborator") . finish_non_exhaustive () } }
    };
}

impl_112!();