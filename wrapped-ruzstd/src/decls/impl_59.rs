macro_rules! deps {
    () => {
        Error!();
        DecompressBlockError!();
        DecodeBlockContentError!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl core :: fmt :: Display for DecodeBlockContentError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { DecodeBlockContentError :: DecoderStateIsFailed => { write ! (f , "Can't decode next block if failed along the way. Results will be nonsense" ,) } DecodeBlockContentError :: ExpectedHeaderOfPreviousBlock => { write ! (f , "Can't decode next block body, while expecting to decode the header of the previous block. Results will be nonsense" ,) } DecodeBlockContentError :: ReadError { step , source } => { write ! (f , "Error while reading bytes for {step}: {source}" ,) } DecodeBlockContentError :: DecompressBlockError (e) => write ! (f , "{e:?}") , } } }
    };
}

impl_59!();