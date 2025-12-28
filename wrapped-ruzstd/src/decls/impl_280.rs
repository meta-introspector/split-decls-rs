macro_rules! deps {
    () => {
        Error!();
        BlockType!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl core :: fmt :: Display for BlockType { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> Result < () , core :: fmt :: Error > { match self { BlockType :: Compressed => write ! (f , "Compressed") , BlockType :: Raw => write ! (f , "Raw") , BlockType :: RLE => write ! (f , "RLE") , BlockType :: Reserved => write ! (f , "Reserverd") , } } }
    };
}

impl_280!()