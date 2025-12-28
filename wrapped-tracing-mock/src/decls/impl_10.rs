macro_rules! deps {
    () => {
        ExpectedEvent!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl fmt :: Display for ExpectedEvent { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "an event{}" , self . metadata) } }
    };
}

impl_10!()