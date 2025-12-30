// Generated macro for impl_65 (impl)
macro_rules! Depcrate_decoding_errorsimpl_65 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_65"}
// Dependencies: {}
impl core :: fmt :: Display for DecompressBlockError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { DecompressBlockError :: BlockContentReadError (e) => { write ! (f , "Error while reading the block content: {e}") } DecompressBlockError :: MalformedSectionHeader { expected_len , remaining_bytes , } => { write ! (f , "Malformed section header. Says literals would be this long: {expected_len} but there are only {remaining_bytes} bytes left" ,) } DecompressBlockError :: DecompressLiteralsError (e) => write ! (f , "{e:?}") , DecompressBlockError :: LiteralsSectionParseError (e) => write ! (f , "{e:?}") , DecompressBlockError :: SequencesHeaderParseError (e) => write ! (f , "{e:?}") , DecompressBlockError :: DecodeSequenceError (e) => write ! (f , "{e:?}") , DecompressBlockError :: ExecuteSequencesError (e) => write ! (f , "{e:?}") , } } }
};
}
