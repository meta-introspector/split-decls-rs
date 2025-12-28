macro_rules! deps {
    () => {
        BlockSizeError!();
        BlockTypeError!();
        Error!();
    };
}

macro_rules! BlockHeaderReadError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum BlockHeaderReadError { ReadError (Error) , FoundReservedBlock , BlockTypeError (BlockTypeError) , BlockSizeError (BlockSizeError) , }
    };
}

BlockHeaderReadError!()