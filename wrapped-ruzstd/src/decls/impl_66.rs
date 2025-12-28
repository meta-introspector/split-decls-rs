macro_rules! deps {
    () => {
        FSETableError!();
        HuffmanTableError!();
        DictionaryDecodeError!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl core :: fmt :: Display for DictionaryDecodeError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { DictionaryDecodeError :: BadMagicNum { got } => { write ! (f , "Bad magic_num at start of the dictionary; Got: {:#04X?}, Expected: {:#04x?}" , got , crate :: decoding :: dictionary :: MAGIC_NUM ,) } DictionaryDecodeError :: FSETableError (e) => write ! (f , "{e:?}") , DictionaryDecodeError :: HuffmanTableError (e) => write ! (f , "{e:?}") , } } }
    };
}

impl_66!();