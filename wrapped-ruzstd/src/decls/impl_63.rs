macro_rules! deps {
    () => {
        DecodeBufferError!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl core :: fmt :: Display for DecodeBufferError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { DecodeBufferError :: NotEnoughBytesInDictionary { got , need } => { write ! (f , "Need {need} bytes from the dictionary but it is only {got} bytes long" ,) } DecodeBufferError :: OffsetTooBig { offset , buf_len } => { write ! (f , "offset: {offset} bigger than buffer: {buf_len}" ,) } } } }
    };
}

impl_63!()