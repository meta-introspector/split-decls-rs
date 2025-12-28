macro_rules! deps {
    () => {
        InlineAsmRegOrRegClass!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl fmt :: Display for InlineAsmRegOrRegClass { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Reg (r) => write ! (f , "\"{}\"" , r . name ()) , Self :: RegClass (r) => write ! (f , "{}" , r . name ()) , } } }
    };
}

impl_118!();