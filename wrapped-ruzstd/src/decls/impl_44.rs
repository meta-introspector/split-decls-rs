macro_rules! deps {
    () => {
        BlockTypeError!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl core :: fmt :: Display for BlockTypeError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { BlockTypeError :: InvalidBlocktypeNumber { num } => { write ! (f , "Invalid Blocktype number. Is: {num} Should be one of: 0, 1, 2, 3 (3 is reserved though" ,) } } } }
    };
}

impl_44!()