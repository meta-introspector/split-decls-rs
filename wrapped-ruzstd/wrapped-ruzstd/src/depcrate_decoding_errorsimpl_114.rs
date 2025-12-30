// Generated macro for impl_114 (impl)
macro_rules! Depcrate_decoding_errorsimpl_114 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_114"}
// Dependencies: {}
impl core :: fmt :: Display for SequencesHeaderParseError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { SequencesHeaderParseError :: NotEnoughBytes { need_at_least , got } => { write ! (f , "source must have at least {need_at_least} bytes to parse header; got {got} bytes" ,) } } } }
};
}
