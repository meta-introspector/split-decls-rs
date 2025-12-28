macro_rules! deps {
    () => {
        FSETableError!();
        DecodeSequenceError!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl From < FSETableError > for DecodeSequenceError { fn from (val : FSETableError) -> Self { Self :: FSETableError (val) } }
    };
}

impl_90!();