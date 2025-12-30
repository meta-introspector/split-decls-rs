// Generated macro for impl_62 (impl)
macro_rules! Depcrate_decoding_errorsimpl_62 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_62"}
// Dependencies: {}
impl core :: fmt :: Display for BlockSizeError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { BlockSizeError :: BlockSizeTooLarge { size } => { write ! (f , "Blocksize was bigger than the absolute maximum {} (128kb). Is: {}" , crate :: common :: MAX_BLOCK_SIZE , size ,) } } } }
};
}
