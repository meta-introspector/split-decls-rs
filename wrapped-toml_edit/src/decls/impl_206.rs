macro_rules! deps {
    () => {
        Error!();
        RawString!();
        RawStringInner!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl std :: fmt :: Debug for RawString { # [inline] fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { match & self . 0 { RawStringInner :: Empty => write ! (formatter , "empty") , RawStringInner :: Explicit (s) => write ! (formatter , "{s:?}") , RawStringInner :: Spanned (s) => write ! (formatter , "{s:?}") , } } }
    };
}

impl_206!();