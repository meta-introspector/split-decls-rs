macro_rules! BlockTypeError {
    () => {
        # [derive (Debug)] # [non_exhaustive] pub enum BlockTypeError { InvalidBlocktypeNumber { num : u8 } , }
    };
}

BlockTypeError!();