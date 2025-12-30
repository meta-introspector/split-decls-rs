// Generated macro for impl_98 (impl)
macro_rules! Depcrate_decoding_errorsimpl_98 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_98"}
// Dependencies: {}
impl core :: fmt :: Display for ExecuteSequencesError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { ExecuteSequencesError :: DecodebufferError (e) => { write ! (f , "{e:?}") } ExecuteSequencesError :: NotEnoughBytesForSequence { wanted , have } => { write ! (f , "Sequence wants to copy up to byte {wanted}. Bytes in literalsbuffer: {have}") } ExecuteSequencesError :: ZeroOffset => { write ! (f , "Illegal offset: 0 found") } } } }
};
}
