macro_rules! deps {
    () => {
        InferTy!();
    };
}

macro_rules! impl_451 {
    () => {
        deps!();
        impl fmt :: Display for InferTy { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use InferTy :: * ; match * self { TyVar (_) => write ! (f , "_") , IntVar (_) => write ! (f , "{}" , "{integer}") , FloatVar (_) => write ! (f , "{}" , "{float}") , FreshTy (v) => write ! (f , "FreshTy({v})") , FreshIntTy (v) => write ! (f , "FreshIntTy({v})") , FreshFloatTy (v) => write ! (f , "FreshFloatTy({v})") , } } }
    };
}

impl_451!()