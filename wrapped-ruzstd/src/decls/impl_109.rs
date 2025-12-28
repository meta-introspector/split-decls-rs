macro_rules! deps {
    () => {
        HuffmanTableError!();
        FSETableError!();
        FSEDecoderError!();
        GetBitsError!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl StdError for HuffmanTableError { fn source (& self) -> Option < & (dyn StdError + 'static) > { match self { HuffmanTableError :: GetBitsError (source) => Some (source) , HuffmanTableError :: FSEDecoderError (source) => Some (source) , HuffmanTableError :: FSETableError (source) => Some (source) , _ => None , } } }
    };
}

impl_109!()