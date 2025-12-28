macro_rules! deps {
    () => {
        HuffmanDecoderError!();
        GetBitsError!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl core :: fmt :: Display for HuffmanDecoderError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { HuffmanDecoderError :: GetBitsError (e) => write ! (f , "{e:?}") , } } }
    };
}

impl_115!();