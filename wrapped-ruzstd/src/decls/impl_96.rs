macro_rules! deps {
    () => {
        LiteralsSectionType!();
        Error!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl core :: fmt :: Display for LiteralsSectionType { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> Result < () , core :: fmt :: Error > { match self { LiteralsSectionType :: Compressed => write ! (f , "Compressed") , LiteralsSectionType :: Raw => write ! (f , "Raw") , LiteralsSectionType :: RLE => write ! (f , "RLE") , LiteralsSectionType :: Treeless => write ! (f , "Treeless") , } } }
    };
}

impl_96!();