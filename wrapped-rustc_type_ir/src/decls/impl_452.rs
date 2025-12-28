macro_rules! deps {
    () => {
        InferTy!();
    };
}

macro_rules! impl_452 {
    () => {
        deps!();
        impl fmt :: Debug for InferTy { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use InferTy :: * ; match * self { TyVar (ref v) => v . fmt (f) , IntVar (ref v) => v . fmt (f) , FloatVar (ref v) => v . fmt (f) , FreshTy (v) => write ! (f , "FreshTy({v:?})") , FreshIntTy (v) => write ! (f , "FreshIntTy({v:?})") , FreshFloatTy (v) => write ! (f , "FreshFloatTy({v:?})") , } } }
    };
}

impl_452!();