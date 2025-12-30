// Generated macro for impl_130 (impl)
macro_rules! Depcrate_decoding_errorsimpl_130 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_130"}
// Dependencies: {}
impl core :: fmt :: Display for HuffmanDecoderError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { HuffmanDecoderError :: GetBitsError (e) => write ! (f , "{e:?}") , } } }
};
}
