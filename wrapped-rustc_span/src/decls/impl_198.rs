macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl fmt :: Debug for Symbol { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . as_str () , f) } }
    };
}

impl_198!()