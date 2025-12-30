// Generated macro for DecompressBlockError (enum)
macro_rules! Depcrate_decoding_errorsDecompressBlockError {
() => {
// Module: crate::decoding::errors
// Provides: {"DecompressBlockError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum DecompressBlockError { BlockContentReadError (Error) , MalformedSectionHeader { expected_len : usize , remaining_bytes : usize , } , DecompressLiteralsError (DecompressLiteralsError) , LiteralsSectionParseError (LiteralsSectionParseError) , SequencesHeaderParseError (SequencesHeaderParseError) , DecodeSequenceError (DecodeSequenceError) , ExecuteSequencesError (ExecuteSequencesError) , }
};
}
