macro_rules! deps {
    () => {
        ByteSymbol!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl fmt :: Debug for ByteSymbol { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . as_byte_str () , f) } }
    };
}

impl_205!();