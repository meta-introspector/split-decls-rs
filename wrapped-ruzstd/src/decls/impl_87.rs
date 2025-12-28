macro_rules! deps {
    () => {
        FSEDecoderError!();
        Error!();
        GetBitsError!();
        DecodeSequenceError!();
        FSETableError!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for DecodeSequenceError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { DecodeSequenceError :: GetBitsError (source) => Some (source) , DecodeSequenceError :: FSEDecoderError (source) => Some (source) , DecodeSequenceError :: FSETableError (source) => Some (source) , _ => None , } } }
    };
}

impl_87!();