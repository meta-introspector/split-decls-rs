// Generated macro for impl_111 (impl)
macro_rules! Depcrate_decoding_errorsimpl_111 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_111"}
// Dependencies: {}
impl core :: fmt :: Display for LiteralsSectionType { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> Result < () , core :: fmt :: Error > { match self { LiteralsSectionType :: Compressed => write ! (f , "Compressed") , LiteralsSectionType :: Raw => write ! (f , "Raw") , LiteralsSectionType :: RLE => write ! (f , "RLE") , LiteralsSectionType :: Treeless => write ! (f , "Treeless") , } } }
};
}
