macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl core :: fmt :: Debug for BSTR { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: write ! (f , "{}" , self . display ()) } }
    };
}

impl_11!()