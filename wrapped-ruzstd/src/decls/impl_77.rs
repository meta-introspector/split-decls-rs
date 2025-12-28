macro_rules! deps {
    () => {
        HuffmanDecoderError!();
        Error!();
        GetBitsError!();
        DecompressLiteralsError!();
        HuffmanTableError!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for DecompressLiteralsError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { DecompressLiteralsError :: GetBitsError (source) => Some (source) , DecompressLiteralsError :: HuffmanTableError (source) => Some (source) , DecompressLiteralsError :: HuffmanDecoderError (source) => Some (source) , _ => None , } } }
    };
}

impl_77!()