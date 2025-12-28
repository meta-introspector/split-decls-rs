macro_rules! deps {
    () => {
        InferConst!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl fmt :: Debug for InferConst { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { InferConst :: Var (var) => write ! (f , "{var:?}") , InferConst :: Fresh (var) => write ! (f , "Fresh({var:?})") , } } }
    };
}

impl_284!();