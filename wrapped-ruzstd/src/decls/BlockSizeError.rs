macro_rules! BlockSizeError {
    () => {
        # [derive (Debug)] # [non_exhaustive] pub enum BlockSizeError { BlockSizeTooLarge { size : u32 } , }
    };
}

BlockSizeError!()