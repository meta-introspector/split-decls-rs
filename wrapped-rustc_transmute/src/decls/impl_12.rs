macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl fmt :: Debug for State { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "S_{}" , self . 0) } }
    };
}

impl_12!();