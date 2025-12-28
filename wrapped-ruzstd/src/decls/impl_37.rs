macro_rules! deps {
    () => {
        BlockSizeError!();
        BlockTypeError!();
        Error!();
        BlockHeaderReadError!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for BlockHeaderReadError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { BlockHeaderReadError :: ReadError (source) => Some (source) , BlockHeaderReadError :: BlockTypeError (source) => Some (source) , BlockHeaderReadError :: BlockSizeError (source) => Some (source) , BlockHeaderReadError :: FoundReservedBlock => None , } } }
    };
}

impl_37!()