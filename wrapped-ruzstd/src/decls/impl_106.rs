macro_rules! deps {
    () => {
        GetBitsError!();
        FSEDecoderError!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl core :: fmt :: Display for FSEDecoderError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { FSEDecoderError :: GetBitsError (e) => write ! (f , "{e:?}") , FSEDecoderError :: TableIsUninitialized => { write ! (f , "Tried to use an uninitialized table!") } } } }
    };
}

impl_106!()