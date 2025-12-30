// Generated macro for impl_121 (impl)
macro_rules! Depcrate_decoding_errorsimpl_121 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_121"}
// Dependencies: {}
impl core :: fmt :: Display for FSEDecoderError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { FSEDecoderError :: GetBitsError (e) => write ! (f , "{e:?}") , FSEDecoderError :: TableIsUninitialized => { write ! (f , "Tried to use an uninitialized table!") } } } }
};
}
