macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl fmt :: Debug for Inner { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . name . is_empty () { write ! (f , "Inner {{...}}") } else { write ! (f , "Inner {{name={}, ...}}" , self . name) } } }
    };
}

impl_12!();