macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl core :: fmt :: Debug for HSTRING { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{}" , self . display ()) } }
    };
}

impl_27!();