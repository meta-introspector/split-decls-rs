macro_rules! deps {
    () => {
        HuffmanDecoderError!();
        GetBitsError!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl StdError for HuffmanDecoderError { fn source (& self) -> Option < & (dyn StdError + 'static) > { match self { HuffmanDecoderError :: GetBitsError (source) => Some (source) , } } }
    };
}

impl_116!();