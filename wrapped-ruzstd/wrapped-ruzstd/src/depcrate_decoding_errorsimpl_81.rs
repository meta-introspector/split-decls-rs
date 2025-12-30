// Generated macro for impl_81 (impl)
macro_rules! Depcrate_decoding_errorsimpl_81 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_81"}
// Dependencies: {}
impl core :: fmt :: Display for DictionaryDecodeError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { DictionaryDecodeError :: BadMagicNum { got } => { write ! (f , "Bad magic_num at start of the dictionary; Got: {:#04X?}, Expected: {:#04x?}" , got , crate :: decoding :: dictionary :: MAGIC_NUM ,) } DictionaryDecodeError :: FSETableError (e) => write ! (f , "{e:?}") , DictionaryDecodeError :: HuffmanTableError (e) => write ! (f , "{e:?}") , } } }
};
}
