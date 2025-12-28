macro_rules! deps {
    () => {
        ExpectedMetadata!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl fmt :: Display for ExpectedMetadata { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (ref name) = self . name { write ! (f , " named `{}`" , name) ? ; } if let Some (ref level) = self . level { write ! (f , " at the `{:?}` level" , level) ? ; } if let Some (ref target) = self . target { write ! (f , " with target `{}`" , target) ? ; } Ok (()) } }
    };
}

impl_42!()