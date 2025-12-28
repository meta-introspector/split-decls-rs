macro_rules! deps {
    () => {
        StrSimError!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl Display for StrSimError { fn fmt (& self , fmt : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { let text = match self { StrSimError :: DifferentLengthArgs => "Differing length arguments provided" , } ; write ! (fmt , "{}" , text) } }
    };
}

impl_1!();