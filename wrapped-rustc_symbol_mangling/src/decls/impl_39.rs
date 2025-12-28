macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl fmt :: Display for Kind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Kind :: SymbolName => write ! (f , "symbol-name") , Kind :: Demangling => write ! (f , "demangling") , Kind :: DemanglingAlt => write ! (f , "demangling-alt") , Kind :: DefPath => write ! (f , "def-path") , } } }
    };
}

impl_39!();