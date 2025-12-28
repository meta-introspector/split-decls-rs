macro_rules! deps {
    () => {
        FSEDecoderError!();
        GetBitsError!();
        Error!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for FSEDecoderError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { FSEDecoderError :: GetBitsError (source) => Some (source) , _ => None , } } }
    };
}

impl_105!()