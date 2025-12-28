macro_rules! deps {
    () => {
        DecodeSequenceError!();
        DecompressBlockError!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl From < DecodeSequenceError > for DecompressBlockError { fn from (val : DecodeSequenceError) -> Self { Self :: DecodeSequenceError (val) } }
    };
}

impl_55!()