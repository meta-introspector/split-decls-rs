macro_rules! deps {
    () => {
        PanicMsgSnippet!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a > fmt :: Display for PanicMsgSnippet < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . 0 . name . is_empty () { write ! (f , "({} actions remain)" , self . 0 . actions . len ()) } else { write ! (f , "(name {}, {} actions remain)" , self . 0 . name , self . 0 . actions . len ()) } } }
    };
}

impl_14!()