macro_rules! deps {
    () => {
        Byte!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl fmt :: Debug for Byte { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . start == Self :: UNINIT && self . end == Self :: UNINIT + 1 { write ! (f , "uninit") } else if self . start <= Self :: UNINIT && self . end == Self :: UNINIT + 1 { write ! (f , "{}..{}|uninit" , self . start , self . end - 1) } else { write ! (f , "{}..{}" , self . start , self . end) } } }
    };
}

impl_23!();