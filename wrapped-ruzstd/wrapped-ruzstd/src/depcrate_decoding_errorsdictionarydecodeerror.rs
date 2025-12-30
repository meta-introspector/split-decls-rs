// Generated macro for DictionaryDecodeError (enum)
macro_rules! Depcrate_decoding_errorsDictionaryDecodeError {
() => {
// Module: crate::decoding::errors
// Provides: {"DictionaryDecodeError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum DictionaryDecodeError { BadMagicNum { got : [u8 ; 4] } , FSETableError (FSETableError) , HuffmanTableError (HuffmanTableError) , }
};
}
