// Generated macro for impl_53 (impl)
macro_rules! Depcrate_decoding_errorsimpl_53 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_53"}
// Dependencies: {}
impl :: core :: fmt :: Display for BlockHeaderReadError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> :: core :: fmt :: Result { match self { BlockHeaderReadError :: ReadError (_) => write ! (f , "Error while reading the block header") , BlockHeaderReadError :: FoundReservedBlock => write ! (f , "Reserved block occured. This is considered corruption by the documentation") , BlockHeaderReadError :: BlockTypeError (e) => write ! (f , "Error getting block type: {e}") , BlockHeaderReadError :: BlockSizeError (e) => { write ! (f , "Error getting block content size: {e}") } } } }
};
}
