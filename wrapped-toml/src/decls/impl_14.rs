macro_rules! deps {
    () => {
        Map!();
        Error!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < K : Debug , V : Debug > Debug for Map < K , V > { # [inline] fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { self . map . fmt (formatter) } }
    };
}

impl_14!()