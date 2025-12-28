macro_rules! deps {
    () => {
        SyntaxContext!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl fmt :: Display for SyntaxContext { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_root () { write ! (f , "ROOT{}" , Edition :: from_u32 (SyntaxContext :: MAX_ID - self . into_u32 ()) . number ()) } else { write ! (f , "{}" , self . into_u32 ()) } } }
    };
}

impl_64!()