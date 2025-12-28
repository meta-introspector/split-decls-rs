macro_rules! deps {
    () => {
        TextRange!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl fmt :: Debug for TextRange { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}..{}" , self . start () . raw , self . end () . raw) } }
    };
}

impl_1!();