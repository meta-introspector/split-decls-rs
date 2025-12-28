macro_rules! deps {
    () => {
        HuffmanTableError!();
        FSETableError!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl From < FSETableError > for HuffmanTableError { fn from (val : FSETableError) -> Self { Self :: FSETableError (val) } }
    };
}

impl_113!()