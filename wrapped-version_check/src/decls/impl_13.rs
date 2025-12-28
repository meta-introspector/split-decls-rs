macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl fmt :: Debug for Version { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Version({:?}, {:?})" , self . 0 , self . to_mmp ()) } }
    };
}

impl_13!();