// Generated macro for impl_407 (impl)
macro_rules! Depcrate_blocks_blockimpl_407 {
() => {
// Module: crate::blocks::block
// Provides: {"impl_407"}
// Dependencies: {}
impl core :: fmt :: Display for BlockType { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> Result < () , core :: fmt :: Error > { match self { BlockType :: Compressed => write ! (f , "Compressed") , BlockType :: Raw => write ! (f , "Raw") , BlockType :: RLE => write ! (f , "RLE") , BlockType :: Reserved => write ! (f , "Reserverd") , } } }
};
}
