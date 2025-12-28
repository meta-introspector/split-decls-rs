macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl fmt :: Debug for TextSize { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . raw) } }
    };
}

impl_20!();