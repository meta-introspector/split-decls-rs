macro_rules! deps {
    () => {
        BlockSizeError!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl core :: fmt :: Display for BlockSizeError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { BlockSizeError :: BlockSizeTooLarge { size } => { write ! (f , "Blocksize was bigger than the absolute maximum {} (128kb). Is: {}" , crate :: common :: MAX_BLOCK_SIZE , size ,) } } } }
    };
}

impl_47!()