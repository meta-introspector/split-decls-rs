macro_rules! deps {
    () => {
        DecompressLiteralsError!();
        DecompressBlockError!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl From < DecompressLiteralsError > for DecompressBlockError { fn from (val : DecompressLiteralsError) -> Self { Self :: DecompressLiteralsError (val) } }
    };
}

impl_52!();