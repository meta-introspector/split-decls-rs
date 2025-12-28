macro_rules! deps {
    () => {
        DictionaryDecodeError!();
        FSETableError!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl From < FSETableError > for DictionaryDecodeError { fn from (val : FSETableError) -> Self { Self :: FSETableError (val) } }
    };
}

impl_67!();