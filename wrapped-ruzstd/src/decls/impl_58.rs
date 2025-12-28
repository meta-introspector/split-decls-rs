macro_rules! deps {
    () => {
        DecompressBlockError!();
        DecodeBlockContentError!();
        Error!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for DecodeBlockContentError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { DecodeBlockContentError :: ReadError { step : _ , source } => Some (source) , DecodeBlockContentError :: DecompressBlockError (source) => Some (source) , _ => None , } } }
    };
}

impl_58!()