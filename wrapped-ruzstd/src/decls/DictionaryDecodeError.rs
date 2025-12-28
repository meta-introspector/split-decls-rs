macro_rules! deps {
    () => {
        FSETableError!();
        HuffmanTableError!();
    };
}

macro_rules! DictionaryDecodeError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum DictionaryDecodeError { BadMagicNum { got : [u8 ; 4] } , FSETableError (FSETableError) , HuffmanTableError (HuffmanTableError) , }
    };
}

DictionaryDecodeError!();