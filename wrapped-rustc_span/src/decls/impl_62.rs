macro_rules! deps {
    () => {
        SyntaxContext!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl fmt :: Debug for SyntaxContext { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "#{}" , self . 0) } }
    };
}

impl_62!();