macro_rules! deps {
    () => {
        FSETableError!();
        DictionaryDecodeError!();
        HuffmanTableError!();
        Error!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for DictionaryDecodeError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { DictionaryDecodeError :: FSETableError (source) => Some (source) , DictionaryDecodeError :: HuffmanTableError (source) => Some (source) , _ => None , } } }
    };
}

impl_65!()