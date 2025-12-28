macro_rules! deps {
    () => {
        ExpectedSpan!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl fmt :: Display for ExpectedSpan { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . metadata . name . is_some () { write ! (f , "a span{}" , self . metadata) } else { write ! (f , "any span{}" , self . metadata) } } }
    };
}

impl_55!();