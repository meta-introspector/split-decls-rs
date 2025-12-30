// Generated macro for impl_59 (impl)
macro_rules! Depcrate_decoding_errorsimpl_59 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_59"}
// Dependencies: {}
impl core :: fmt :: Display for BlockTypeError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { BlockTypeError :: InvalidBlocktypeNumber { num } => { write ! (f , "Invalid Blocktype number. Is: {num} Should be one of: 0, 1, 2, 3 (3 is reserved though" ,) } } } }
};
}
