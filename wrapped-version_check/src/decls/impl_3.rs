macro_rules! deps {
    () => {
        Channel!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl fmt :: Display for Channel { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , self . as_str ()) } }
    };
}

impl_3!();